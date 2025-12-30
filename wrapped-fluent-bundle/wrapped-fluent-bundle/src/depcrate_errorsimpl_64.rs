// Generated macro for impl_64 (impl)
macro_rules! Depcrate_errorsimpl_64 {
() => {
// Module: crate::errors
// Provides: {"impl_64"}
// Dependencies: {}
impl std :: fmt :: Display for EntryKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Message => f . write_str ("message") , Self :: Term => f . write_str ("term") , Self :: Function => f . write_str ("function") , } } }
};
}
