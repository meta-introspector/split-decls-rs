// Generated macro for impl_63 (impl)
macro_rules! Depcrate_serimpl_63 {
() => {
// Module: crate::ser
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a , W : Write > ser :: SerializeMap for CollectionSerializer < 'a , W > where W :: Error : core :: fmt :: Debug , { type Ok = () ; type Error = Error < W :: Error > ; # [inline] fn serialize_key < U : ? Sized + ser :: Serialize > (& mut self , key : & U) -> Result < () , Self :: Error > { key . serialize (& mut * self . encoder) } # [inline] fn serialize_value < U : ? Sized + ser :: Serialize > (& mut self , value : & U ,) -> Result < () , Self :: Error > { value . serialize (& mut * self . encoder) } end ! () ; }
};
}
