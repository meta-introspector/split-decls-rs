// Generated macro for impl_9 (impl)
macro_rules! Depcrate_adapter_stripimpl_9 {
() => {
// Module: crate::adapter::strip
// Provides: {"impl_9"}
// Dependencies: {}
impl std :: fmt :: Display for StrippedStr < '_ > { # [doc = " **Note:** this does *not* exhaust the [`Iterator`]"] # [inline] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let iter = Self { bytes : self . bytes , state : self . state , } ; for printable in iter { printable . fmt (f) ? ; } Ok (()) } }
};
}
