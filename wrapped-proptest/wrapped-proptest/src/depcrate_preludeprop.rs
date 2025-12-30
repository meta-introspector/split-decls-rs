// Generated macro for prop (module)
macro_rules! Depcrate_preludeprop {
() => {
// Module: crate::prelude
// Provides: {"prop"}
// Dependencies: {}
# [doc = " Re-exports the entire public API of proptest so that an import of `prelude`"] # [doc = " allows simply writing, for example, `prop::num::i32::ANY` rather than"] # [doc = " `proptest::num::i32::ANY` plus a separate `use proptest;`."] pub mod prop { pub use crate :: arbitrary ; pub use crate :: array ; pub use crate :: bits ; pub use crate :: bool ; pub use crate :: char ; pub use crate :: collection ; pub use crate :: num ; pub use crate :: option ; pub use crate :: result ; pub use crate :: sample ; pub use crate :: strategy ; # [cfg (feature = "std")] pub use crate :: string ; pub use crate :: test_runner ; pub use crate :: tuple ; }
};
}
