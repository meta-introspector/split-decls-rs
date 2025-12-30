// Generated macro for prelude (macro)
macro_rules! Depcrate_macrosprelude {
() => {
// Module: crate::macros
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " Create an internal crate prelude with `core` reexports and common types."] macro_rules ! prelude { () => { # [doc = " Frequently-used types that are available on all platforms"] # [doc = ""] # [doc = " We need to reexport the core types so this works with `rust-dep-of-std`."] mod prelude { # [allow (unused_imports)] pub (crate) use :: core :: clone :: Clone ; # [allow (unused_imports)] pub (crate) use :: core :: marker :: { Copy , Send , Sync } ; # [allow (unused_imports)] pub (crate) use :: core :: option :: Option ; # [allow (unused_imports)] pub (crate) use :: core :: { fmt , hash , iter , mem } ; # [allow (unused_imports)] pub (crate) use mem :: { align_of , align_of_val , size_of , size_of_val } ; # [allow (unused_imports)] pub (crate) use crate :: { c_char , c_double , c_float , c_int , c_long , c_longlong , c_short , c_uchar , c_uint , c_ulong , c_ulonglong , c_ushort , c_void , intptr_t , size_t , ssize_t , uintptr_t , } ; } } ; }
};
}
