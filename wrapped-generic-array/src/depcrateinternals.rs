// Generated macro for internals (module)
macro_rules! Depcrateinternals {
() => {
// Module: crate
// Provides: {"internals"}
// Dependencies: {}
# [cfg (feature = "internals")] pub mod internals { # ! [doc = " Very unsafe internal functionality."] # ! [doc = ""] # ! [doc = " These are used internally for building and consuming generic arrays. When used correctly,"] # ! [doc = " they can ensure elements are correctly dropped if something panics while using them."] # ! [doc = ""] # ! [doc = " The API of these is not guaranteed to be stable, as they are not intended for general use."] pub use crate :: internal :: { IntrusiveArrayBuilder , IntrusiveArrayConsumer } ; pub use crate :: internal :: { ArrayBuilder , ArrayConsumer } ; }
};
}
