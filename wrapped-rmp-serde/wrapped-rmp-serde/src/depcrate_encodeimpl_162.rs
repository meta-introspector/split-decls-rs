// Generated macro for impl_162 (impl)
macro_rules! Depcrate_encodeimpl_162 {
() => {
// Module: crate::encode
// Provides: {"impl_162"}
// Dependencies: {}
impl < 'a , W : Write + 'a , C : SerializerConfig > SerializeStruct for Compound < 'a , W , C > { type Ok = () ; type Error = Error ; # [inline] fn serialize_field < T : ? Sized + Serialize > (& mut self , key : & 'static str , value : & T ,) -> Result < () , Self :: Error > { if self . se . config . is_named { encode :: write_str (self . se . get_mut () , key) ? ; } value . serialize (& mut * self . se) } # [inline (always)] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
