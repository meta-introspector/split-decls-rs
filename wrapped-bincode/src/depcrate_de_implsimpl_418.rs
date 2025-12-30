// Generated macro for impl_418 (impl)
macro_rules! Depcrate_de_implsimpl_418 {
() => {
// Module: crate::de::impls
// Provides: {"impl_418"}
// Dependencies: {}
impl < Context , T : Decode < Context > > Decode < Context > for Reverse < T > { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { Ok (Reverse (T :: decode (decoder) ?)) } }
};
}
