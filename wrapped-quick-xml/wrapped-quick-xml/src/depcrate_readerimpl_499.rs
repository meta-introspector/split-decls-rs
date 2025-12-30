// Generated macro for impl_499 (impl)
macro_rules! Depcrate_readerimpl_499 {
() => {
// Module: crate::reader
// Provides: {"impl_499"}
// Dependencies: {}
# [doc = " Builder methods"] impl < R > Reader < R > { # [doc = " Creates a `Reader` that reads from a given reader."] pub fn from_reader (reader : R) -> Self { Self { reader , state : ReaderState :: default () , } } # [doc = " Returns reference to the parser configuration"] pub const fn config (& self) -> & Config { & self . state . config } # [doc = " Returns mutable reference to the parser configuration"] pub fn config_mut (& mut self) -> & mut Config { & mut self . state . config } }
};
}
