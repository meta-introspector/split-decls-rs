// Generated macro for impl_1130 (impl)
macro_rules! Depcrate_httpimpl_1130 {
() => {
// Module: crate::http
// Provides: {"impl_1130"}
// Dependencies: {}
impl < T > Serialize for GraphQLResponse < T > where T : Serialize + ScalarValue , Value < T > : Serialize , ExecutionError < T > : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { match self . 0 { Ok ((ref res , ref err)) => { let mut map = serializer . serialize_map (None) ? ; map . serialize_key ("data") ? ; map . serialize_value (res) ? ; if ! err . is_empty () { map . serialize_key ("errors") ? ; map . serialize_value (err) ? ; } map . end () } Err (ref err) => { let mut map = serializer . serialize_map (Some (1)) ? ; map . serialize_key ("errors") ? ; map . serialize_value (err) ? ; map . end () } } } }
};
}
