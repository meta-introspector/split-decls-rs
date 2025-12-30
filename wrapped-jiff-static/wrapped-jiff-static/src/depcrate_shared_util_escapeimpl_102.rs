// Generated macro for impl_102 (impl)
macro_rules! Depcrate_shared_util_escapeimpl_102 {
() => {
// Module: crate::shared::util::escape
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a > core :: fmt :: Debug for Bytes < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "\"") ? ; core :: fmt :: Display :: fmt (self , f) ? ; write ! (f , "\"") ? ; Ok (()) } }
};
}
