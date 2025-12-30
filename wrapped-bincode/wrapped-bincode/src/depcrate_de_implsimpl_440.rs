// Generated macro for impl_440 (impl)
macro_rules! Depcrate_de_implsimpl_440 {
() => {
// Module: crate::de::impls
// Provides: {"impl_440"}
// Dependencies: {}
impl < Context , T > Decode < Context > for Range < T > where T : Decode < Context > , { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { let min = T :: decode (decoder) ? ; let max = T :: decode (decoder) ? ; Ok (min .. max) } }
};
}
