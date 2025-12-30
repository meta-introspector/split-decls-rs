// Generated macro for io (module)
macro_rules! Depcrateio {
() => {
// Module: crate
// Provides: {"io"}
// Dependencies: {}
# [doc = " Subset of `std::io` which is used as part of borsh public API."] # [doc = ""] # [doc = " When crate is built with `std` feature disabled (it’s enabled by default),"] # [doc = " the exported types are custom borsh types which try to mimic behaviour of"] # [doc = " corresponding standard types usually offering subset of features."] pub mod io { pub use super :: io_impl :: { Error , ErrorKind , Read , Result , Write } ; }
};
}
