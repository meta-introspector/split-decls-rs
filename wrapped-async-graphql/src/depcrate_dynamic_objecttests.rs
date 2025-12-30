// Generated macro for tests (module)
macro_rules! Depcrate_dynamic_objecttests {
() => {
// Module: crate::dynamic::object
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { Value , dynamic :: * , value } ; # [tokio :: test] async fn borrow_context () { struct MyObjData { value : i32 , } let my_obj = Object :: new ("MyObj") . field (Field :: new ("value" , TypeRef :: named (TypeRef :: INT) , | ctx | { FieldFuture :: new (async move { Ok (Some (Value :: from (ctx . parent_value . try_downcast_ref :: < MyObjData > () ? . value ,))) }) })) ; let query = Object :: new ("Query") . field (Field :: new ("obj" , TypeRef :: named_nn (my_obj . type_name ()) , | ctx | { FieldFuture :: new (async move { Ok (Some (FieldValue :: borrowed_any (ctx . data_unchecked :: < MyObjData > () ,))) }) } ,)) ; let schema = Schema :: build ("Query" , None , None) . register (query) . register (my_obj) . data (MyObjData { value : 123 }) . finish () . unwrap () ; assert_eq ! (schema . execute ("{ obj { value } }") . await . into_result () . unwrap () . data , value ! ({ "obj" : { "value" : 123 , } })) ; } }
};
}
