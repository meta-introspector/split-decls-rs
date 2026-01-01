# AST Trace: ../rust/compiler/rustc_middle/src/hooks/mod.rs

Generated 7 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=6

```rust
// "Hooks" let you write `tcx` methods in downstream crates and call them in this crate, reducing
// the amount of code that needs to be in this crate (which is already very big). This is somewhat
// similar to queries, but queries come with a lot of machinery for caching and incremental
// compilation, whereas hooks are just plain function pointers without any of the query magic.

use crate::rustc_complete::def_id::{DefId, DefPathHash};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::rustc_complete::StableCrateId;
use crate::rustc_complete::def_id::{CrateNum, LocalDefId};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::rustc_complete::{ExpnHash, ExpnId};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::mir;
use crate::ty::{Ty, TyCtxt};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=$name | COMPLEXITY=24 | LINES=36

```rust
macro_rules! declare_hooks {
    ($($(#[$attr:meta])*hook $name:ident($($arg:ident: $K:ty),*) -> $V:ty;)*) => {

        impl<'tcx> TyCtxt<'tcx> {
            $(
            $(#[$attr])*
            #[inline(always)]
            pub fn $name(self, $($arg: $K,)*) -> $V
            {
                (self.hooks.$name)(self, $($arg,)*)
            }
            )*
        }

        pub struct Providers {
            $(pub $name: for<'tcx> fn(
                TyCtxt<'tcx>,
                $($arg: $K,)*
            ) -> $V,)*
        }

        impl Default for Providers {
            fn default() -> Self {
                Providers {
                    $($name: |_, $($arg,)*| default_hook(stringify!($name), &($($arg,)*))),*
                }
            }
        }

        impl Copy for Providers {}
        impl Clone for Providers {
            fn clone(&self) -> Self { *self }
        }
    };
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=23 | LINES=57

```rust
declare_hooks! {
    /// Tries to destructure an `mir::Const` ADT or array into its variant index
    /// and its field values. This should only be used for pretty printing.
    hook try_destructure_mir_constant_for_user_output(val: mir::ConstValue, ty: Ty<'tcx>) -> Option<mir::DestructuredConstant<'tcx>>;

    /// Getting a &core::panic::Location referring to a span.
    hook const_caller_location(file: crate::rustc_span::Symbol, line: u32, col: u32) -> mir::ConstValue;

    /// Returns `true` if this def is a function-like thing that is eligible for
    /// coverage instrumentation under `-Cinstrument-coverage`.
    ///
    /// (Eligible functions might nevertheless be skipped for other reasons.)
    hook is_eligible_for_coverage(key: LocalDefId) -> bool;

    /// Imports all `SourceFile`s from the given crate into the current session.
    /// This normally happens automatically when we decode a `Span` from
    /// that crate's metadata - however, the incr comp cache needs
    /// to trigger this manually when decoding a foreign `Span`
    hook import_source_files(key: CrateNum) -> ();

    hook expn_hash_to_expn_id(
        cnum: CrateNum,
        index_guess: u32,
        hash: ExpnHash
    ) -> ExpnId;

    /// Converts a `DefPathHash` to its corresponding `DefId` in the current compilation
    /// session, if it still exists. This is used during incremental compilation to
    /// turn a deserialized `DefPathHash` into its current `DefId`.
    /// Will fetch a DefId from a DefPathHash for a foreign crate.
    hook def_path_hash_to_def_id_extern(hash: DefPathHash, stable_crate_id: StableCrateId) -> DefId;

    /// Returns `true` if we should codegen an instance in the local crate, or returns `false` if we
    /// can just link to the upstream crate and therefore don't need a mono item.
    ///
    /// Note: this hook isn't called within `rustc_middle` but #127779 suggests it's a hook instead
    /// of a normal function because external tools might want to override it.
    hook should_codegen_locally(instance: crate::ty::Instance<'tcx>) -> bool;

    hook alloc_self_profile_query_strings() -> ();

    /// Saves and writes the DepGraph to the file system.
    ///
    /// This function saves both the dep-graph and the query result cache,
    /// and drops the result cache.
    ///
    /// This function should only run after all queries have completed.
    /// Trying to execute a query afterwards would attempt to read the result cache we just dropped.
    hook save_dep_graph() -> ();

    hook query_key_hash_verify_all() -> ();

    /// Ensure the given scalar is valid for the given type.
    /// This checks non-recursive runtime validity.
    hook validate_scalar_in_layout(scalar: crate::ty::ScalarInt, ty: Ty<'tcx>) -> bool;
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=default_hook | COMPLEXITY=5 | LINES=7

```rust
#[cold]
fn default_hook(name: &str, args: &dyn std::fmt::Debug) -> ! {
    bug!(
        "`tcx.{name}{args:?}` cannot be called as `{name}` was never assigned to a provider function"
    )
}
```

---
*Generated by AST tracing system*
