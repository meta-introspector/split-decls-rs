// Generated macro for impl_436 (impl)
macro_rules! Depcrate_de_implsimpl_436 {
() => {
// Module: crate::de::impls
// Provides: {"impl_436"}
// Dependencies: {}
impl < Context , T > Decode < Context > for RefCell < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let t = T :: decode (decoder) ? ; Ok (RefCell :: new (t)) } }
};
}
