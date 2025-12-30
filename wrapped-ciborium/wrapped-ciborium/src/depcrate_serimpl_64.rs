// Generated macro for impl_64 (impl)
macro_rules! Depcrate_serimpl_64 {
() => {
// Module: crate::ser
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , W : Write > ser :: SerializeStruct for CollectionSerializer < 'a , W > where W :: Error : core :: fmt :: Debug , { type Ok = () ; type Error = Error < W :: Error > ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , key : & 'static str , value : & U ,) -> Result < () , Self :: Error > { key . serialize (& mut * self . encoder) ? ; value . serialize (& mut * self . encoder) ? ; Ok (()) } end ! () ; }
};
}
