# AST Trace: ../rust/compiler/rustc_codegen_ssa/src/traits/type_.rs

Generated 10 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_abi::{AddressSpace, Float, Integer, Primitive, Reg, Scalar};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use rustc_middle::bug;
use rustc_middle::ty::Ty;
use rustc_middle::ty::layout::{HasTyCtxt, HasTypingEnv, TyAndLayout};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_target::callconv::{ArgAbi, CastTarget, FnAbi};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=type_i8 | COMPLEXITY=6 | LINES=36

```rust
use super::BackendTypes;
use super::misc::MiscCodegenMethods;
use crate::common::TypeKind;
use crate::mir::place::PlaceRef;

pub trait BaseTypeCodegenMethods: BackendTypes {
    fn type_i8(&self) -> Self::Type;
    fn type_i16(&self) -> Self::Type;
    fn type_i32(&self) -> Self::Type;
    fn type_i64(&self) -> Self::Type;
    fn type_i128(&self) -> Self::Type;
    fn type_isize(&self) -> Self::Type;

    fn type_f16(&self) -> Self::Type;
    fn type_f32(&self) -> Self::Type;
    fn type_f64(&self) -> Self::Type;
    fn type_f128(&self) -> Self::Type;

    fn type_array(&self, ty: Self::Type, len: u64) -> Self::Type;
    fn type_func(&self, args: &[Self::Type], ret: Self::Type) -> Self::Type;
    fn type_kind(&self, ty: Self::Type) -> TypeKind;
    fn type_ptr(&self) -> Self::Type;
    fn type_ptr_ext(&self, address_space: AddressSpace) -> Self::Type;
    fn element_type(&self, ty: Self::Type) -> Self::Type;

    /// Returns the number of elements in `self` if it is an LLVM vector type.
    fn vector_length(&self, ty: Self::Type) -> usize;

    fn float_width(&self, ty: Self::Type) -> usize;

    /// Retrieves the bit width of the integer type `self`.
    fn int_width(&self, ty: Self::Type) -> u64;

    fn val_ty(&self, v: Self::Value) -> Self::Type;
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=type_int | COMPLEXITY=33 | LINES=64

```rust
pub trait DerivedTypeCodegenMethods<'tcx>:
    BaseTypeCodegenMethods + MiscCodegenMethods<'tcx> + HasTyCtxt<'tcx> + HasTypingEnv<'tcx>
{
    fn type_int(&self) -> Self::Type {
        match &self.sess().target.c_int_width {
            16 => self.type_i16(),
            32 => self.type_i32(),
            64 => self.type_i64(),
            width => bug!("Unsupported c_int_width: {}", width),
        }
    }

    fn type_from_integer(&self, i: Integer) -> Self::Type {
        use Integer::*;
        match i {
            I8 => self.type_i8(),
            I16 => self.type_i16(),
            I32 => self.type_i32(),
            I64 => self.type_i64(),
            I128 => self.type_i128(),
        }
    }

    fn type_from_float(&self, f: Float) -> Self::Type {
        use Float::*;
        match f {
            F16 => self.type_f16(),
            F32 => self.type_f32(),
            F64 => self.type_f64(),
            F128 => self.type_f128(),
        }
    }

    fn type_needs_drop(&self, ty: Ty<'tcx>) -> bool {
        ty.needs_drop(self.tcx(), self.typing_env())
    }

    fn type_is_sized(&self, ty: Ty<'tcx>) -> bool {
        ty.is_sized(self.tcx(), self.typing_env())
    }

    fn type_is_freeze(&self, ty: Ty<'tcx>) -> bool {
        ty.is_freeze(self.tcx(), self.typing_env())
    }

    fn type_from_primitive(&self, p: Primitive) -> Self::Type {
        use Primitive::*;
        match p {
            Int(i, _) => self.type_from_integer(i),
            Float(f) => self.type_from_float(f),
            Pointer(address_space) => self.type_ptr_ext(address_space),
        }
    }

    fn type_from_scalar(&self, s: Scalar) -> Self::Type {
        // `MaybeUninit` being `repr(transparent)` somewhat implies that the type
        // of a scalar has to be the type of its primitive (which is true in LLVM,
        // where noundef is a parameter attribute or metadata) but if we ever get
        // a backend where that's no longer true, every use of this will need to
        // to carefully scrutinized and re-evaluated.
        self.type_from_primitive(s.primitive())
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=5

```rust
impl<'tcx, T> DerivedTypeCodegenMethods<'tcx> for T where
    Self: BaseTypeCodegenMethods + MiscCodegenMethods<'tcx> + HasTyCtxt<'tcx> + HasTypingEnv<'tcx>
{
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=backend_type | COMPLEXITY=11 | LINES=42

```rust
pub trait LayoutTypeCodegenMethods<'tcx>: BackendTypes {
    /// The backend type used for a rust type when it's in memory,
    /// such as when it's stack-allocated or when it's being loaded or stored.
    fn backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type;
    fn cast_backend_type(&self, ty: &CastTarget) -> Self::Type;
    fn fn_decl_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type;
    fn fn_ptr_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type;
    fn reg_backend_type(&self, ty: &Reg) -> Self::Type;
    /// The backend type used for a rust type when it's in an SSA register.
    ///
    /// For nearly all types this is the same as the [`Self::backend_type`], however
    /// `bool` (and other `0`-or-`1` values) are kept as `i1` in registers but as
    /// [`BaseTypeCodegenMethods::type_i8`] in memory.
    ///
    /// Converting values between the two different backend types is done using
    /// [`from_immediate`](super::BuilderMethods::from_immediate) and
    /// [`to_immediate_scalar`](super::BuilderMethods::to_immediate_scalar).
    fn immediate_backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type;
    fn is_backend_immediate(&self, layout: TyAndLayout<'tcx>) -> bool;
    fn is_backend_scalar_pair(&self, layout: TyAndLayout<'tcx>) -> bool;
    fn scalar_pair_element_backend_type(
        &self,
        layout: TyAndLayout<'tcx>,
        index: usize,
        immediate: bool,
    ) -> Self::Type;

    /// A type that produces an [`OperandValue::Ref`] when loaded.
    ///
    /// AKA one that's not a ZST, not `is_backend_immediate`, and
    /// not `is_backend_scalar_pair`. For such a type, a
    /// [`load_operand`] doesn't actually `load` anything.
    ///
    /// [`OperandValue::Ref`]: crate::mir::operand::OperandValue::Ref
    /// [`load_operand`]: super::BuilderMethods::load_operand
    fn is_backend_ref(&self, layout: TyAndLayout<'tcx>) -> bool {
        !(layout.is_zst()
            || self.is_backend_immediate(layout)
            || self.is_backend_scalar_pair(layout))
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=add_type_metadata | COMPLEXITY=8 | LINES=12

```rust
// For backends that support CFI using type membership (i.e., testing whether a given pointer is
// associated with a type identifier).
pub trait TypeMembershipCodegenMethods<'tcx>: BackendTypes {
    fn add_type_metadata(&self, _function: Self::Function, _typeid: &[u8]) {}
    fn set_type_metadata(&self, _function: Self::Function, _typeid: &[u8]) {}
    fn typeid_metadata(&self, _typeid: &[u8]) -> Option<Self::Metadata> {
        None
    }
    fn add_kcfi_type_metadata(&self, _function: Self::Function, _typeid: u32) {}
    fn set_kcfi_type_metadata(&self, _function: Self::Function, _typeid: u32) {}
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=store_fn_arg | COMPLEXITY=2 | LINES=15

```rust
pub trait ArgAbiBuilderMethods<'tcx>: BackendTypes {
    fn store_fn_arg(
        &mut self,
        arg_abi: &ArgAbi<'tcx, Ty<'tcx>>,
        idx: &mut usize,
        dst: PlaceRef<'tcx, Self::Value>,
    );
    fn store_arg(
        &mut self,
        arg_abi: &ArgAbi<'tcx, Ty<'tcx>>,
        val: Self::Value,
        dst: PlaceRef<'tcx, Self::Value>,
    );
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=1 | LINES=4

```rust
pub trait TypeCodegenMethods<'tcx> = DerivedTypeCodegenMethods<'tcx>
    + LayoutTypeCodegenMethods<'tcx>
    + TypeMembershipCodegenMethods<'tcx>;
```

---
*Generated by AST tracing system*
