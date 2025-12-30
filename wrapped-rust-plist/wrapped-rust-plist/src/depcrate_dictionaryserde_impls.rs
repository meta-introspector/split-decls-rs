// Generated macro for serde_impls (module)
macro_rules! Depcrate_dictionaryserde_impls {
() => {
// Module: crate::dictionary
// Provides: {"serde_impls"}
// Dependencies: {}
# [cfg (feature = "serde")] pub mod serde_impls { use serde :: { de , ser } ; use std :: fmt ; use crate :: Dictionary ; impl ser :: Serialize for Dictionary { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { use serde :: ser :: SerializeMap ; let mut map = serializer . serialize_map (Some (self . len ())) ? ; for (k , v) in self { map . serialize_key (k) ? ; map . serialize_value (v) ? ; } map . end () } } impl < 'de > de :: Deserialize < 'de > for Dictionary { # [inline] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct Visitor ; impl < 'de > de :: Visitor < 'de > for Visitor { type Value = Dictionary ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a map") } # [inline] fn visit_unit < E > (self) -> Result < Self :: Value , E > where E : de :: Error , { Ok (Dictionary :: new ()) } # [inline] fn visit_map < V > (self , mut visitor : V) -> Result < Self :: Value , V :: Error > where V : de :: MapAccess < 'de > , { let mut values = Dictionary :: new () ; while let Some ((key , value)) = visitor . next_entry () ? { values . insert (key , value) ; } Ok (values) } } deserializer . deserialize_map (Visitor) } } }
};
}
