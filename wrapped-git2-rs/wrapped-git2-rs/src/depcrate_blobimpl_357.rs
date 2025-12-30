// Generated macro for impl_357 (impl)
macro_rules! Depcrate_blobimpl_357 {
() => {
// Module: crate::blob
// Provides: {"impl_357"}
// Dependencies: {}
impl < 'repo > std :: fmt :: Debug for Blob < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("Blob") . field ("id" , & self . id ()) . finish () } }
};
}
