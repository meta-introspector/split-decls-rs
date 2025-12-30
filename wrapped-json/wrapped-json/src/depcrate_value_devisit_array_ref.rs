// Generated macro for visit_array_ref (function)
macro_rules! Depcrate_value_devisit_array_ref {
() => {
// Module: crate::value::de
// Provides: {"visit_array_ref"}
// Dependencies: {}
fn visit_array_ref < 'de , V > (array : & 'de [Value] , visitor : V) -> Result < V :: Value , Error > where V : Visitor < 'de > , { let len = array . len () ; let mut deserializer = SeqRefDeserializer :: new (array) ; let seq = tri ! (visitor . visit_seq (& mut deserializer)) ; let remaining = deserializer . iter . len () ; if remaining == 0 { Ok (seq) } else { Err (serde :: de :: Error :: invalid_length (len , & "fewer elements in array" ,)) } }
};
}
