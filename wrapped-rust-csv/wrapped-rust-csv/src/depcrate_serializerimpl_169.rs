// Generated macro for impl_169 (impl)
macro_rules! Depcrate_serializerimpl_169 {
() => {
// Module: crate::serializer
// Provides: {"impl_169"}
// Dependencies: {}
impl < 'a , 'w , W : io :: Write > SerializeStruct for & 'a mut SeHeader < 'w , W > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized + Serialize > (& mut self , key : & 'static str , value : & T ,) -> Result < () , Self :: Error > { let old_state = mem :: replace (& mut self . state , HeaderState :: EncounteredStructField) ; if let HeaderState :: ErrorIfWrite (err) = old_state { return Err (err) ; } self . wtr . write_field (key) ? ; self . state = HeaderState :: InStructField ; value . serialize (& mut * * self) ? ; self . state = HeaderState :: EncounteredStructField ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (()) } }
};
}
