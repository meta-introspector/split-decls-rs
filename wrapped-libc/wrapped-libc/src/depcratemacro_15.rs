// Generated macro for macro_15 (macro)
macro_rules! Depcratemacro_15 {
() => {
// Module: crate
// Provides: {"macro_15"}
// Dependencies: {}
cfg_if ! { if # [cfg (libc_priv_mod_use)] { # [cfg (libc_core_cvoid)] # [allow (unused_imports)] use core :: ffi ; # [allow (unused_imports)] use core :: fmt ; # [allow (unused_imports)] use core :: hash ; # [allow (unused_imports)] use core :: num ; # [allow (unused_imports)] use core :: mem ; # [doc (hidden)] # [allow (unused_imports)] use core :: clone :: Clone ; # [doc (hidden)] # [allow (unused_imports)] use core :: marker :: { Copy , Send , Sync } ; # [doc (hidden)] # [allow (unused_imports)] use core :: option :: Option ; } else { # [doc (hidden)] # [allow (unused_imports)] pub use core :: fmt ; # [doc (hidden)] # [allow (unused_imports)] pub use core :: hash ; # [doc (hidden)] # [allow (unused_imports)] pub use core :: num ; # [doc (hidden)] # [allow (unused_imports)] pub use core :: mem ; # [doc (hidden)] # [allow (unused_imports)] pub use core :: clone :: Clone ; # [doc (hidden)] # [allow (unused_imports)] pub use core :: marker :: { Copy , Send , Sync } ; # [doc (hidden)] # [allow (unused_imports)] pub use core :: option :: Option ; } }
};
}
