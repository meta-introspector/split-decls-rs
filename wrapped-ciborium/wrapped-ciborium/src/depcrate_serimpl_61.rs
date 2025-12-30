// Generated macro for impl_61 (impl)
macro_rules! Depcrate_serimpl_61 {
() => {
// Module: crate::ser
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a , W : Write > ser :: SerializeTupleStruct for CollectionSerializer < 'a , W > where W :: Error : core :: fmt :: Debug , { type Ok = () ; type Error = Error < W :: Error > ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , value : & U ,) -> Result < () , Self :: Error > { value . serialize (& mut * self . encoder) } end ! () ; }
};
}
