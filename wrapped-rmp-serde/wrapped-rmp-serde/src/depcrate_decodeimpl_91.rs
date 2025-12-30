// Generated macro for impl_91 (impl)
macro_rules! Depcrate_decodeimpl_91 {
() => {
// Module: crate::decode
// Provides: {"impl_91"}
// Dependencies: {}
impl < 'de , 'a , R : ReadSlice < 'de > + 'a , C : SerializerConfig > ExtDeserializer < 'a , R , C > { const fn new (d : & 'a mut Deserializer < R , C > , len : u32) -> Self { ExtDeserializer { rd : & mut d . rd , _config : d . _config , len , state : ExtDeserializerState :: New , } } }
};
}
