// Generated macro for impl_62 (impl)
macro_rules! Depcrate_serimpl_62 {
() => {
// Module: crate::ser
// Provides: {"impl_62"}
// Dependencies: {}
impl < 'a , W : Write > ser :: SerializeTupleVariant for CollectionSerializer < 'a , W > where W :: Error : core :: fmt :: Debug , { type Ok = () ; type Error = Error < W :: Error > ; # [inline] fn serialize_field < U : ? Sized + ser :: Serialize > (& mut self , value : & U ,) -> Result < () , Self :: Error > { if ! self . tag { return value . serialize (& mut * self . encoder) ; } self . tag = false ; match value . serialize (crate :: tag :: Serializer) { Ok (x) => Ok (self . encoder . 0 . push (Header :: Tag (x)) ?) , _ => Err (Error :: Value ("expected tag" . into ())) , } } end ! () ; }
};
}
