// Generated macro for export (module)
macro_rules! Depcrateexport {
() => {
// Module: crate
// Provides: {"export"}
// Dependencies: {}
# [doc = " Core/std trait re-exports. This should help produce generated code which doesn't"] # [doc = " depend on `std` unnecessarily, and avoids problems caused by aliasing `std` or any"] # [doc = " of the referenced types."] # [doc (hidden)] pub mod export { pub use core :: convert :: { identity , From , Into } ; pub use core :: default :: Default ; pub use core :: iter :: IntoIterator ; pub use core :: option :: Option :: { self , None , Some } ; pub use core :: result :: Result :: { self , Err , Ok } ; pub use darling_core :: syn ; pub use std :: string :: ToString ; pub use std :: vec :: Vec ; pub use crate :: ast :: NestedMeta ; }
};
}
