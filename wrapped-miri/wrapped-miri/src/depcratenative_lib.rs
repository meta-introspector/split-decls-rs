// Generated macro for native_lib (module)
macro_rules! Depcratenative_lib {
() => {
// Module: crate
// Provides: {"native_lib"}
// Dependencies: {}
# [cfg (all (unix , feature = "native-lib"))] pub mod native_lib { pub use crate :: shims :: { init_sv , register_retcode_sv } ; }
};
}
