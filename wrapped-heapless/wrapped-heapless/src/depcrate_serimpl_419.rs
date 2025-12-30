// Generated macro for impl_419 (impl)
macro_rules! Depcrate_serimpl_419 {
() => {
// Module: crate::ser
// Provides: {"impl_419"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > Serialize for StringInner < LenT , S > { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { serializer . serialize_str (self) } }
};
}
