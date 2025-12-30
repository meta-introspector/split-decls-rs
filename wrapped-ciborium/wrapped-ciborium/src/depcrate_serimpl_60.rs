// Generated macro for impl_60 (impl)
macro_rules! Depcrate_serimpl_60 {
() => {
// Module: crate::ser
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , W : Write > ser :: SerializeTuple for CollectionSerializer < 'a , W > where W :: Error : core :: fmt :: Debug , { type Ok = () ; type Error = Error < W :: Error > ; # [inline] fn serialize_element < U : ? Sized + ser :: Serialize > (& mut self , value : & U ,) -> Result < () , Self :: Error > { value . serialize (& mut * self . encoder) } end ! () ; }
};
}
