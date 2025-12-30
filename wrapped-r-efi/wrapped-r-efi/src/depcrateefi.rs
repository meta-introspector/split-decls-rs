// Generated macro for efi (module)
macro_rules! Depcrateefi {
() => {
// Module: crate
// Provides: {"efi"}
// Dependencies: {}
# [doc = " Flat EFI Namespace"] # [doc = ""] # [doc = " The EFI namespace re-exports all symbols in a single, flat namespace. This allows mirroring"] # [doc = " the entire EFI namespace as given in the specification and makes it easier to refer to them"] # [doc = " with the same names as the reference C implementation."] # [doc = ""] # [doc = " Note that the individual protocols are put into submodules. The specification does this in"] # [doc = " most parts as well (by prefixing all symbols). This is not true in all cases, as the"] # [doc = " specification suffers from lack of namespaces in the reference C language. However, we decided"] # [doc = " to namespace the remaining bits as well, for better consistency throughout the API. This"] # [doc = " should be self-explanatory in nearly all cases."] pub mod efi { pub use crate :: base :: * ; pub use crate :: system :: * ; pub use crate :: hii ; pub use crate :: protocols ; pub use crate :: vendor ; }
};
}
