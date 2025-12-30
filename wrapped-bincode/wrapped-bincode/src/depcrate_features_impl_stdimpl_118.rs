// Generated macro for impl_118 (impl)
macro_rules! Depcrate_features_impl_stdimpl_118 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_118"}
// Dependencies: {}
impl < T > Encode for RwLock < T > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { let t = self . read () . map_err (| _ | EncodeError :: LockFailed { type_name : core :: any :: type_name :: < RwLock < T > > () , }) ? ; t . encode (encoder) } }
};
}
