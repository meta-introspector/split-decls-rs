// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl < T > fmt :: Debug for State < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Created => f . write_str ("Created") , Self :: Notified { additional , .. } => f . debug_struct ("Notified") . field ("additional" , additional) . finish () , Self :: Task (_) => f . write_str ("Task(_)") , Self :: NotifiedTaken => f . write_str ("NotifiedTaken") , } } }
};
}
