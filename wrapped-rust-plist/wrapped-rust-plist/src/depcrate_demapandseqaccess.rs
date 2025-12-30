// Generated macro for MapAndSeqAccess (struct)
macro_rules! Depcrate_deMapAndSeqAccess {
() => {
// Module: crate::de
// Provides: {"MapAndSeqAccess"}
// Dependencies: {}
struct MapAndSeqAccess < 'a , 'event , I > where I : 'a + IntoIterator < Item = Result < Event < 'event > , Error > > , { de : & 'a mut Deserializer < 'event , I > , is_struct : bool , remaining : Option < usize > , }
};
}
