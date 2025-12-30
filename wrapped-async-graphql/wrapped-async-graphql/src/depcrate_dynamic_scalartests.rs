// Generated macro for tests (module)
macro_rules! Depcrate_dynamic_scalartests {
() => {
// Module: crate::dynamic::scalar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use async_graphql_parser :: Pos ; use crate :: { PathSegment , ServerError , dynamic :: * , value } ; # [tokio :: test] async fn custom_scalar () { let scalar = Scalar :: new ("MyScalar") ; let query = Object :: new ("Query") . field (Field :: new ("value" , TypeRef :: named_nn (scalar . type_name ()) , | _ | { FieldFuture :: new (async move { Ok (Some (value ! ({ "a" : 1 , "b" : "abc" , }))) }) } ,)) ; let schema = Schema :: build (query . type_name () , None , None) . register (query) . register (scalar) . finish () . unwrap () ; assert_eq ! (schema . execute ("{ value }") . await . into_result () . unwrap () . data , value ! ({ "value" : { "a" : 1 , "b" : "abc" , } })) ; } # [tokio :: test] async fn invalid_scalar_value () { let scalar = Scalar :: new ("MyScalar") ; let query = Object :: new ("Query") . field (Field :: new ("value" , TypeRef :: named_nn (scalar . type_name ()) , | _ | FieldFuture :: new (async move { Ok (Some (FieldValue :: owned_any (10i32))) }) ,)) ; let schema = Schema :: build (query . type_name () , None , None) . register (query) . register (scalar) . finish () . unwrap () ; assert_eq ! (schema . execute ("{ value }") . await . into_result () . unwrap_err () , vec ! [ServerError { message : "internal: invalid value for scalar \"MyScalar\", expected \"FieldValue::Value\"" . to_owned () , source : None , locations : vec ! [Pos { column : 3 , line : 1 }] , path : vec ! [PathSegment :: Field ("value" . to_owned ())] , extensions : None , }]) ; } }
};
}
