// Generated macro for impl_434 (impl)
macro_rules! Depcrate_de_implsimpl_434 {
() => {
// Module: crate::de::impls
// Provides: {"impl_434"}
// Dependencies: {}
impl < Context , T > Decode < Context > for Cell < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (Cell :: new (t)) } }
};
}
