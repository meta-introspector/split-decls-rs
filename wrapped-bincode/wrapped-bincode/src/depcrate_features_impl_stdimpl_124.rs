// Generated macro for impl_124 (impl)
macro_rules! Depcrate_features_impl_stdimpl_124 {
() => {
// Module: crate::features::impl_std
// Provides: {"impl_124"}
// Dependencies: {}
impl Encode for & '_ Path { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { match self . to_str () { Some (str) => str . encode (encoder) , None => Err (EncodeError :: InvalidPathCharacters) , } } }
};
}
