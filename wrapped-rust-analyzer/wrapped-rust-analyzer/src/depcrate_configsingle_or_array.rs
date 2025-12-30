// Generated macro for single_or_array (module)
macro_rules! Depcrate_configsingle_or_array {
() => {
// Module: crate::config
// Provides: {"single_or_array"}
// Dependencies: {}
mod single_or_array { use serde :: { Deserialize , Serialize } ; pub (super) fn deserialize < 'de , D > (deserializer : D) -> Result < Vec < String > , D :: Error > where D : serde :: Deserializer < 'de > , { struct SingleOrVec ; impl < 'de > serde :: de :: Visitor < 'de > for SingleOrVec { type Value = Vec < String > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { formatter . write_str ("string or array of strings") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { Ok (vec ! [value . to_owned ()]) } fn visit_seq < A > (self , seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { Deserialize :: deserialize (serde :: de :: value :: SeqAccessDeserializer :: new (seq)) } } deserializer . deserialize_any (SingleOrVec) } pub (super) fn serialize < S > (vec : & [String] , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { match vec { [single] => serializer . serialize_str (single) , slice => slice . serialize (serializer) , } } }
};
}
