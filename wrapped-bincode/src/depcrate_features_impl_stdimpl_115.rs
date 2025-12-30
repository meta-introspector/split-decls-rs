// Generated macro for impl_115 (impl)
macro_rules! Depcrate_features_impl_stdimpl_115 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_115"}
// Dependencies: {}
impl < T > Encode for Mutex < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { let t = self . lock () . map_err (| _ | EncodeError :: LockFailed { type_name : core :: any :: type_name :: < Mutex < T > > () , }) ? ; t . encode (encoder) } }
};
}
