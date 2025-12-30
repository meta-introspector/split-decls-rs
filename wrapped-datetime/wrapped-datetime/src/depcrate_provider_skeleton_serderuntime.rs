// Generated macro for runtime (module)
macro_rules! Depcrate_provider_skeleton_serderuntime {
() => {
// Module: crate::provider::skeleton::serde
// Provides: {"runtime"}
// Dependencies: {}
pub mod runtime { use super :: super :: runtime :: Skeleton ; use super :: * ; use :: serde :: { ser , Serialize } ; use serde :: { de , Deserialize , Deserializer } ; use zerovec :: ZeroVec ; # [doc = " This is an implementation of the serde deserialization visitor pattern."] struct DeserializeSkeletonUTS35String ; impl < 'de > de :: Visitor < 'de > for DeserializeSkeletonUTS35String { type Value = Skeleton < 'de > ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (formatter , "Expected to find a valid skeleton.") } fn visit_borrowed_str < E > (self , skeleton_string : & 'de str) -> Result < Self :: Value , E > where E : de :: Error , { let reference_deserializer = super :: reference :: DeserializeSkeletonUTS35String ; let skeleton = reference_deserializer . visit_str (skeleton_string) ? ; Ok (skeleton . into ()) } } impl < 'de > Deserialize < 'de > for Skeleton < 'de > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { deserializer . deserialize_str (DeserializeSkeletonUTS35String) } else { let zv = ZeroVec :: deserialize (deserializer) ? ; Ok (zv . into ()) } } } impl Serialize for Skeleton < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { if serializer . is_human_readable () { let string = self . to_string () ; serializer . serialize_str (& string) } else { self . 0 . serialize (serializer) } } } }
};
}
