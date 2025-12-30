// Generated macro for is_serde_content (function)
macro_rules! Depcrate_deis_serde_content {
() => {
// Module: crate::de
// Provides: {"is_serde_content"}
// Dependencies: {}
fn is_serde_content < T > () -> bool { # [derive (serde_derive :: Deserialize)] enum A { } type B = A ; # [derive (serde_derive :: Deserialize)] # [serde (untagged)] enum UntaggedEnum { A (A) , B (B) , } struct TypeIdDeserializer ; impl < 'de > de :: Deserializer < 'de > for TypeIdDeserializer { type Error = TypeIdError ; fn deserialize_any < V : Visitor < 'de > > (self , _visitor : V) -> Result < V :: Value , Self :: Error > { Err (TypeIdError (typeid :: of :: < V :: Value > ())) } serde :: forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } } # [derive (Debug)] struct TypeIdError (core :: any :: TypeId) ; impl core :: fmt :: Display for TypeIdError { fn fmt (& self , _fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { Ok (()) } } impl de :: Error for TypeIdError { # [allow (clippy :: unreachable)] fn custom < T : core :: fmt :: Display > (_msg : T) -> Self { unreachable ! () } } impl de :: StdError for TypeIdError { } fn type_id_of_untagged_enum_default_buffer () -> core :: any :: TypeId { static TYPE_ID : once_cell :: race :: OnceBox < core :: any :: TypeId > = once_cell :: race :: OnceBox :: new () ; * TYPE_ID . get_or_init (| | match Deserialize :: deserialize (TypeIdDeserializer) { Ok (UntaggedEnum :: A (void) | UntaggedEnum :: B (void)) => match void { } , Err (TypeIdError (typeid)) => alloc :: boxed :: Box :: new (typeid) , }) } typeid :: of :: < T > () == type_id_of_untagged_enum_default_buffer () }
};
}
