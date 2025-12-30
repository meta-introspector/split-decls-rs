// Generated macro for impl_578 (impl)
macro_rules! Depcrate_se_simple_typeimpl_578 {
() => {
// Module: crate::se::simple_type
// Provides: {"impl_578"}
// Dependencies: {}
impl < W : Write > SerializeTuple for SimpleSeq < W > { type Ok = W ; type Error = SeError ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { SerializeSeq :: serialize_element (self , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { SerializeSeq :: end (self) } }
};
}
