// Generated macro for impl_416 (impl)
macro_rules! Depcrate_de_implsimpl_416 {
() => {
// Module: crate::de::impls
// Provides: {"impl_416"}
// Dependencies: {}
impl < Context , T : Decode < Context > > Decode < Context > for Wrapping < T > { fn decode < D : Decoder < Context = Context > > (decoder : & mut D) -> Result < Self , DecodeError > { Ok (Wrapping (T :: decode (decoder) ?)) } }
};
}
