# AST Trace: ../rust/compiler/rustc_middle/src/ty/codec.rs

Generated 66 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=11

```rust
// This module contains some shared code for encoding and decoding various
// things from the `ty` module, and in particular implements support for
// "shorthands" which allow to have pointers back into the already encoded
// stream instead of re-encoding the same thing twice.
//
// The functionality in here is shared between persisting to crate metadata and
// persisting to incr. comp. caches.

use std::hash::Hash;
use std::intrinsics;
use std::marker::{DiscriminantKind, PointeeSized};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::rustc_abi::{FieldIdx, VariantIdx};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::rustc_data_structures::fx::FxHashMap;
use crate::rustc_complete::def_id::LocalDefId;
use crate::rustc_serialize::{Decodable, Encodable};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::rustc_complete::source_map::Spanned;
use crate::rustc_complete::{Span, SpanDecoder, SpanEncoder};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::arena::ArenaAllocatable;
use crate::infer::canonical::{CanonicalVarKind, CanonicalVarKinds};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::mir::interpret::{AllocId, ConstAllocation, CtfeProvenance};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::mir::mono::MonoItem;
use crate::mir::{self};
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::traits;
use crate::ty::{self, AdtDef, GenericArgsRef, Ty, TyCtxt};
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=position | COMPLEXITY=3 | LINES=17

```rust
/// The shorthand encoding uses an enum's variant index `usize`
/// and is offset by this value so it never matches a real variant.
/// This offset is also chosen so that the first byte is never < 0x80.
pub const SHORTHAND_OFFSET: usize = 0x80;

pub trait TyEncoder<'tcx>: SpanEncoder {
    const CLEAR_CROSS_CRATE: bool;

    fn position(&self) -> usize;

    fn type_shorthands(&mut self) -> &mut FxHashMap<Ty<'tcx>, usize>;

    fn predicate_shorthands(&mut self) -> &mut FxHashMap<ty::PredicateKind<'tcx>, usize>;

    fn encode_alloc_id(&mut self, alloc_id: &AllocId);
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=interner | COMPLEXITY=4 | LINES=20

```rust
pub trait TyDecoder<'tcx>: SpanDecoder {
    const CLEAR_CROSS_CRATE: bool;

    fn interner(&self) -> TyCtxt<'tcx>;

    fn cached_ty_for_shorthand<F>(&mut self, shorthand: usize, or_insert_with: F) -> Ty<'tcx>
    where
        F: FnOnce(&mut Self) -> Ty<'tcx>;

    fn with_position<F, R>(&mut self, pos: usize, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R;

    fn positioned_at_shorthand(&self) -> bool {
        (self.peek_byte() & (SHORTHAND_OFFSET as u8)) != 0
    }

    fn decode_alloc_id(&mut self) -> AllocId;
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=variant | COMPLEXITY=2 | LINES=5

```rust
pub trait EncodableWithShorthand<'tcx, E: TyEncoder<'tcx>>: Copy + Eq + Hash {
    type Variant: Encodable<E>;
    fn variant(&self) -> &Self::Variant;
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=variant | COMPLEXITY=5 | LINES=10

```rust
#[allow(rustc::usage_of_ty_tykind)]
impl<'tcx, E: TyEncoder<'tcx>> EncodableWithShorthand<'tcx, E> for Ty<'tcx> {
    type Variant = ty::TyKind<'tcx>;

    #[inline]
    fn variant(&self) -> &Self::Variant {
        self.kind()
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=variant | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, E: TyEncoder<'tcx>> EncodableWithShorthand<'tcx, E> for ty::PredicateKind<'tcx> {
    type Variant = ty::PredicateKind<'tcx>;

    #[inline]
    fn variant(&self) -> &Self::Variant {
        self
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=7 | LINES=14

```rust
/// Trait for decoding to a reference.
///
/// This is a separate trait from `Decodable` so that we can implement it for
/// upstream types, such as `FxHashSet`.
///
/// The `TyDecodable` derive macro will use this trait for fields that are
/// references (and don't use a type alias to hide that).
///
/// `Decodable` can still be implemented in cases where `Decodable` is required
/// by a trait bound.
pub trait RefDecodable<'tcx, D: TyDecoder<'tcx>>: PointeeSized {
    fn decode(d: &mut D) -> &'tcx Self;
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=encode_with_shorthand | COMPLEXITY=10 | LINES=39

```rust
/// Encode the given value or a previously cached shorthand.
pub fn encode_with_shorthand<'tcx, E, T, M>(encoder: &mut E, value: &T, cache: M)
where
    E: TyEncoder<'tcx>,
    M: for<'b> Fn(&'b mut E) -> &'b mut FxHashMap<T, usize>,
    T: EncodableWithShorthand<'tcx, E>,
    // The discriminant and shorthand must have the same size.
    T::Variant: DiscriminantKind<Discriminant = isize>,
{
    let existing_shorthand = cache(encoder).get(value).copied();
    if let Some(shorthand) = existing_shorthand {
        encoder.emit_usize(shorthand);
        return;
    }

    let variant = value.variant();

    let start = encoder.position();
    variant.encode(encoder);
    let len = encoder.position() - start;

    // The shorthand encoding uses the same usize as the
    // discriminant, with an offset so they can't conflict.
    let discriminant = intrinsics::discriminant_value(variant);
    assert!(SHORTHAND_OFFSET > discriminant as usize);

    let shorthand = start + SHORTHAND_OFFSET;

    // Get the number of bits that leb128 could fit
    // in the same space as the fully encoded type.
    let leb128_bits = len * 7;

    // Check that the shorthand is a not longer than the
    // full encoding itself, i.e., it's an obvious win.
    if leb128_bits >= 64 || (shorthand as u64) < (1 << leb128_bits) {
        cache(encoder).insert(*value, shorthand);
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for Ty<'tcx> {
    fn encode(&self, e: &mut E) {
        encode_with_shorthand(e, self, TyEncoder::type_shorthands);
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=8

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for ty::Predicate<'tcx> {
    fn encode(&self, e: &mut E) {
        let kind = self.kind();
        kind.bound_vars().encode(e);
        encode_with_shorthand(e, &kind.skip_binder(), TyEncoder::predicate_shorthands);
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for ty::Clause<'tcx> {
    fn encode(&self, e: &mut E) {
        self.as_predicate().encode(e);
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for ty::Region<'tcx> {
    fn encode(&self, e: &mut E) {
        self.kind().encode(e);
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for ty::Const<'tcx> {
    fn encode(&self, e: &mut E) {
        self.0.0.encode(e);
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for ty::Pattern<'tcx> {
    fn encode(&self, e: &mut E) {
        self.0.0.encode(e);
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for ty::ValTree<'tcx> {
    fn encode(&self, e: &mut E) {
        self.0.0.encode(e);
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for ConstAllocation<'tcx> {
    fn encode(&self, e: &mut E) {
        self.inner().encode(e)
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for AdtDef<'tcx> {
    fn encode(&self, e: &mut E) {
        self.0.0.encode(e)
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for AllocId {
    fn encode(&self, e: &mut E) {
        e.encode_alloc_id(self)
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for CtfeProvenance {
    fn encode(&self, e: &mut E) {
        self.into_parts().encode(e);
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, E: TyEncoder<'tcx>> Encodable<E> for ty::ParamEnv<'tcx> {
    fn encode(&self, e: &mut E) {
        self.caller_bounds().encode(e);
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=decode_arena_allocable | COMPLEXITY=2 | LINES=7

```rust
#[inline]
fn decode_arena_allocable<'tcx, D: TyDecoder<'tcx>, T: ArenaAllocatable<'tcx> + Decodable<D>>(
    decoder: &mut D,
) -> &'tcx T {
    decoder.interner().arena.alloc(Decodable::decode(decoder))
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=decode_arena_allocable_slice | COMPLEXITY=2 | LINES=11

```rust
#[inline]
fn decode_arena_allocable_slice<
    'tcx,
    D: TyDecoder<'tcx>,
    T: ArenaAllocatable<'tcx> + Decodable<D>,
>(
    decoder: &mut D,
) -> &'tcx [T] {
    decoder.interner().arena.alloc_from_iter(<Vec<T> as Decodable<D>>::decode(decoder))
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=13 | LINES=19

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for Ty<'tcx> {
    #[allow(rustc::usage_of_ty_tykind)]
    fn decode(decoder: &mut D) -> Ty<'tcx> {
        // Handle shorthands first, if we have a usize > 0x80.
        if decoder.positioned_at_shorthand() {
            let pos = decoder.read_usize();
            assert!(pos >= SHORTHAND_OFFSET);
            let shorthand = pos - SHORTHAND_OFFSET;

            decoder.cached_ty_for_shorthand(shorthand, |decoder| {
                decoder.with_position(shorthand, Ty::decode)
            })
        } else {
            let tcx = decoder.interner();
            tcx.mk_ty_from_kind(ty::TyKind::decode(decoder))
        }
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=12 | LINES=20

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for ty::Predicate<'tcx> {
    fn decode(decoder: &mut D) -> ty::Predicate<'tcx> {
        let bound_vars = Decodable::decode(decoder);
        // Handle shorthands first, if we have a usize > 0x80.
        let predicate_kind = ty::Binder::bind_with_vars(
            if decoder.positioned_at_shorthand() {
                let pos = decoder.read_usize();
                assert!(pos >= SHORTHAND_OFFSET);
                let shorthand = pos - SHORTHAND_OFFSET;

                decoder.with_position(shorthand, <ty::PredicateKind<'tcx> as Decodable<D>>::decode)
            } else {
                <ty::PredicateKind<'tcx> as Decodable<D>>::decode(decoder)
            },
            bound_vars,
        );
        decoder.interner().mk_predicate(predicate_kind)
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for ty::Clause<'tcx> {
    fn decode(decoder: &mut D) -> ty::Clause<'tcx> {
        let pred: ty::Predicate<'tcx> = Decodable::decode(decoder);
        pred.expect_clause()
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=10

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for GenericArgsRef<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        let len = decoder.read_usize();
        let tcx = decoder.interner();
        tcx.mk_args_from_iter(
            (0..len).map::<ty::GenericArg<'tcx>, _>(|_| Decodable::decode(decoder)),
        )
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=6 | LINES=11

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for mir::Place<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        let local: mir::Local = Decodable::decode(decoder);
        let len = decoder.read_usize();
        let projection = decoder.interner().mk_place_elems_from_iter(
            (0..len).map::<mir::PlaceElem<'tcx>, _>(|_| Decodable::decode(decoder)),
        );
        mir::Place { local, projection }
    }
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for ty::Region<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        ty::Region::new_from_kind(decoder.interner(), Decodable::decode(decoder))
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for CanonicalVarKinds<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        let len = decoder.read_usize();
        decoder.interner().mk_canonical_var_infos_from_iter(
            (0..len).map::<CanonicalVarKind<'tcx>, _>(|_| Decodable::decode(decoder)),
        )
    }
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for AllocId {
    fn decode(decoder: &mut D) -> Self {
        decoder.decode_alloc_id()
    }
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for CtfeProvenance {
    fn decode(decoder: &mut D) -> Self {
        let parts = Decodable::decode(decoder);
        CtfeProvenance::from_parts(parts)
    }
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for ty::SymbolName<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        ty::SymbolName::new(decoder.interner(), decoder.read_str())
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for ty::ParamEnv<'tcx> {
    fn decode(d: &mut D) -> Self {
        let caller_bounds = Decodable::decode(d);
        ty::ParamEnv::new(caller_bounds)
    }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=12 | LINES=10

```rust
macro_rules! impl_decodable_via_ref {
    ($($t:ty,)+) => {
        $(impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for $t {
            fn decode(decoder: &mut D) -> Self {
                RefDecodable::decode(decoder)
            }
        })*
    }
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for ty::List<Ty<'tcx>> {
    fn decode(decoder: &mut D) -> &'tcx Self {
        let len = decoder.read_usize();
        decoder
            .interner()
            .mk_type_list_from_iter((0..len).map::<Ty<'tcx>, _>(|_| Decodable::decode(decoder)))
    }
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=11

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D>
    for ty::List<ty::PolyExistentialPredicate<'tcx>>
{
    fn decode(decoder: &mut D) -> &'tcx Self {
        let len = decoder.read_usize();
        decoder.interner().mk_poly_existential_predicates_from_iter(
            (0..len).map::<ty::Binder<'tcx, _>, _>(|_| Decodable::decode(decoder)),
        )
    }
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for ty::Const<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        let kind: ty::ConstKind<'tcx> = Decodable::decode(decoder);
        decoder.interner().mk_ct_from_kind(kind)
    }
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for ty::Pattern<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        decoder.interner().mk_pat(Decodable::decode(decoder))
    }
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for ty::ValTree<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        decoder.interner().intern_valtree(Decodable::decode(decoder))
    }
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for ConstAllocation<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        decoder.interner().mk_const_alloc(Decodable::decode(decoder))
    }
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for AdtDef<'tcx> {
    fn decode(decoder: &mut D) -> Self {
        decoder.interner().mk_adt_def_from_data(Decodable::decode(decoder))
    }
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for [(ty::Clause<'tcx>, Span)] {
    fn decode(decoder: &mut D) -> &'tcx Self {
        decoder
            .interner()
            .arena
            .alloc_from_iter((0..decoder.read_usize()).map(|_| Decodable::decode(decoder)))
    }
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for [(ty::PolyTraitRef<'tcx>, Span)] {
    fn decode(decoder: &mut D) -> &'tcx Self {
        decoder
            .interner()
            .arena
            .alloc_from_iter((0..decoder.read_usize()).map(|_| Decodable::decode(decoder)))
    }
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for [Spanned<MonoItem<'tcx>>] {
    fn decode(decoder: &mut D) -> &'tcx Self {
        decoder
            .interner()
            .arena
            .alloc_from_iter((0..decoder.read_usize()).map(|_| Decodable::decode(decoder)))
    }
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for ty::List<ty::BoundVariableKind> {
    fn decode(decoder: &mut D) -> &'tcx Self {
        let len = decoder.read_usize();
        decoder.interner().mk_bound_variable_kinds_from_iter(
            (0..len).map::<ty::BoundVariableKind, _>(|_| Decodable::decode(decoder)),
        )
    }
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for ty::List<ty::Pattern<'tcx>> {
    fn decode(decoder: &mut D) -> &'tcx Self {
        let len = decoder.read_usize();
        decoder.interner().mk_patterns_from_iter(
            (0..len).map::<ty::Pattern<'tcx>, _>(|_| Decodable::decode(decoder)),
        )
    }
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for ty::List<ty::Const<'tcx>> {
    fn decode(decoder: &mut D) -> &'tcx Self {
        let len = decoder.read_usize();
        decoder.interner().mk_const_list_from_iter(
            (0..len).map::<ty::Const<'tcx>, _>(|_| Decodable::decode(decoder)),
        )
    }
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=11

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D>
    for ty::ListWithCachedTypeInfo<ty::Clause<'tcx>>
{
    fn decode(decoder: &mut D) -> &'tcx Self {
        let len = decoder.read_usize();
        decoder.interner().mk_clauses_from_iter(
            (0..len).map::<ty::Clause<'tcx>, _>(|_| Decodable::decode(decoder)),
        )
    }
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for ty::List<FieldIdx> {
    fn decode(decoder: &mut D) -> &'tcx Self {
        let len = decoder.read_usize();
        decoder
            .interner()
            .mk_fields_from_iter((0..len).map::<FieldIdx, _>(|_| Decodable::decode(decoder)))
    }
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for ty::List<LocalDefId> {
    fn decode(decoder: &mut D) -> &'tcx Self {
        let len = decoder.read_usize();
        decoder.interner().mk_local_def_ids_from_iter(
            (0..len).map::<LocalDefId, _>(|_| Decodable::decode(decoder)),
        )
    }
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx, D: TyDecoder<'tcx>> Decodable<D> for &'tcx ty::List<LocalDefId> {
    fn decode(d: &mut D) -> Self {
        RefDecodable::decode(d)
    }
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for ty::List<(VariantIdx, FieldIdx)> {
    fn decode(decoder: &mut D) -> &'tcx Self {
        let len = decoder.read_usize();
        decoder.interner().mk_offset_of_from_iter(
            (0..len).map::<(VariantIdx, FieldIdx), _>(|_| Decodable::decode(decoder)),
        )
    }
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
impl_decodable_via_ref! {
    &'tcx ty::TypeckResults<'tcx>,
    &'tcx ty::List<Ty<'tcx>>,
    &'tcx ty::List<ty::PolyExistentialPredicate<'tcx>>,
    &'tcx traits::ImplSource<'tcx, ()>,
    &'tcx mir::Body<'tcx>,
    &'tcx ty::List<ty::BoundVariableKind>,
    &'tcx ty::List<ty::Pattern<'tcx>>,
    &'tcx ty::ListWithCachedTypeInfo<ty::Clause<'tcx>>,
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=FUNCTION | NAME=$name | COMPLEXITY=9 | LINES=12

```rust
#[macro_export]
macro_rules! __impl_decoder_methods {
    ($($name:ident -> $ty:ty;)*) => {
        $(
            #[inline]
            fn $name(&mut self) -> $ty {
                self.opaque.$name()
            }
        )*
    }
}
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=18 | LINES=20

```rust
macro_rules! impl_arena_allocatable_decoder {
    ([]$args:tt) => {};
    ([decode $(, $attrs:ident)*]
     [$name:ident: $ty:ty]) => {
        impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for $ty {
            #[inline]
            fn decode(decoder: &mut D) -> &'tcx Self {
                decode_arena_allocable(decoder)
            }
        }

        impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for [$ty] {
            #[inline]
            fn decode(decoder: &mut D) -> &'tcx Self {
                decode_arena_allocable_slice(decoder)
            }
        }
    };
}
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=8

```rust
macro_rules! impl_arena_allocatable_decoders {
    ([$($a:tt $name:ident: $ty:ty,)*]) => {
        $(
            impl_arena_allocatable_decoder!($a [$name: $ty]);
        )*
    }
}
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=17 | LINES=21

```rust
crate::rustc_hir::arena_types!(impl_arena_allocatable_decoders);
arena_types!(impl_arena_allocatable_decoders);

macro_rules! impl_arena_copy_decoder {
    (<$tcx:tt> $($ty:ty,)*) => {
        $(impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for $ty {
            #[inline]
            fn decode(decoder: &mut D) -> &'tcx Self {
                decoder.interner().arena.alloc(Decodable::decode(decoder))
            }
        }

        impl<'tcx, D: TyDecoder<'tcx>> RefDecodable<'tcx, D> for [$ty] {
            #[inline]
            fn decode(decoder: &mut D) -> &'tcx Self {
                decoder.interner().arena.alloc_from_iter(<Vec<_> as Decodable<D>>::decode(decoder))
            }
        })*
    };
}
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
impl_arena_copy_decoder! {<'tcx>
    Span,
    crate::rustc_span::Ident,
    ty::Variance,
    crate::rustc_span::def_id::DefId,
    crate::rustc_span::def_id::LocalDefId,
    (crate::rustc_middle::middle::exported_symbols::ExportedSymbol<'tcx>, crate::rustc_middle::middle::exported_symbols::SymbolExportInfo),
    ty::DeducedParamAttrs,
}
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=FUNCTION | NAME=read_raw_bytes | COMPLEXITY=18 | LINES=43

```rust
#[macro_export]
macro_rules! implement_ty_decoder {
    ($DecoderName:ident <$($typaram:tt),*>) => {
        mod __ty_decoder_impl {
            use crate::rustc_serialize::Decoder;

            use super::$DecoderName;

            impl<$($typaram ),*> Decoder for $DecoderName<$($typaram),*> {
                $crate::__impl_decoder_methods! {
                    read_usize -> usize;
                    read_u128 -> u128;
                    read_u64 -> u64;
                    read_u32 -> u32;
                    read_u16 -> u16;
                    read_u8 -> u8;

                    read_isize -> isize;
                    read_i128 -> i128;
                    read_i64 -> i64;
                    read_i32 -> i32;
                    read_i16 -> i16;
                }

                #[inline]
                fn read_raw_bytes(&mut self, len: usize) -> &[u8] {
                    self.opaque.read_raw_bytes(len)
                }

                #[inline]
                fn peek_byte(&self) -> u8 {
                    self.opaque.peek_byte()
                }

                #[inline]
                fn position(&self) -> usize {
                    self.opaque.position()
                }
            }
        }
    }
}
```

---
*Generated by AST tracing system*
