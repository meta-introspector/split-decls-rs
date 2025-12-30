// Generated macro for impl_95 (impl)
macro_rules! Depcrate_jsontimpl_95 {
() => {
// Module: crate::jsont
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a > serde :: Serialize for Message < 'a > { fn serialize < S : serde :: Serializer > (& self , s : S ,) -> Result < S :: Ok , S :: Error > { use serde :: ser :: SerializeStruct ; let mut state = s . serialize_struct ("Message" , 2) ? ; match * self { Message :: Begin (ref msg) => { state . serialize_field ("type" , & "begin") ? ; state . serialize_field ("data" , msg) ? ; } Message :: End (ref msg) => { state . serialize_field ("type" , & "end") ? ; state . serialize_field ("data" , msg) ? ; } Message :: Match (ref msg) => { state . serialize_field ("type" , & "match") ? ; state . serialize_field ("data" , msg) ? ; } Message :: Context (ref msg) => { state . serialize_field ("type" , & "context") ? ; state . serialize_field ("data" , msg) ? ; } } state . end () } }
};
}
