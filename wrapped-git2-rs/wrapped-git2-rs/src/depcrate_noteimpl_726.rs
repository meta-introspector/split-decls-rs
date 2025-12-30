// Generated macro for impl_726 (impl)
macro_rules! Depcrate_noteimpl_726 {
() => {
// Module: crate::note
// Provides: {"impl_726"}
// Dependencies: {}
impl < 'repo > std :: fmt :: Debug for Note < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("Note") . field ("id" , & self . id ()) . finish () } }
};
}
