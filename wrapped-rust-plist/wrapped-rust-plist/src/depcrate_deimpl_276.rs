// Generated macro for impl_276 (impl)
macro_rules! Depcrate_deimpl_276 {
() => {
// Module: crate::de
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'a , 'event , I > MapAndSeqAccess < 'a , 'event , I > where I : 'a + IntoIterator < Item = Result < Event < 'event > , Error > > , { fn new (de : & 'a mut Deserializer < 'event , I > , is_struct : bool , len : Option < usize > ,) -> MapAndSeqAccess < 'a , 'event , I > { MapAndSeqAccess { de , is_struct , remaining : len , } } }
};
}
