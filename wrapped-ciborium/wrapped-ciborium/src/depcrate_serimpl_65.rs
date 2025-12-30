// Generated macro for impl_65 (impl)
macro_rules! Depcrate_serimpl_65 {
() => {
// Module: crate::ser
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a , W : Write > ser :: SerializeStructVariant for CollectionSerializer < 'a , W > where W :: Error : core :: fmt :: Debug , { type Ok = () ; type Error = Error < W :: Error > ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , key : & 'static str , value : & U ,) -> Result < () , Self :: Error > { key . serialize (& mut * self . encoder) ? ; value . serialize (& mut * self . encoder) } end ! () ; }
};
}
