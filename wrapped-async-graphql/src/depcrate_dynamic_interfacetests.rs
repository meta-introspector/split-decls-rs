// Generated macro for tests (module)
macro_rules! Depcrate_dynamic_interfacetests {
() => {
// Module: crate::dynamic::interface
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use async_graphql_parser :: Pos ; use crate :: { PathSegment , ServerError , Value , dynamic :: * , value } ; # [tokio :: test] async fn basic_interface () { let obj_a = Object :: new ("MyObjA") . implement ("MyInterface") . field (Field :: new ("a" , TypeRef :: named (TypeRef :: INT) , | _ | { FieldFuture :: new (async { Ok (Some (Value :: from (100))) }) })) . field (Field :: new ("b" , TypeRef :: named (TypeRef :: INT) , | _ | { FieldFuture :: new (async { Ok (Some (Value :: from (200))) }) })) ; let obj_b = Object :: new ("MyObjB") . implement ("MyInterface") . field (Field :: new ("a" , TypeRef :: named (TypeRef :: INT) , | _ | { FieldFuture :: new (async { Ok (Some (Value :: from (300))) }) })) . field (Field :: new ("c" , TypeRef :: named (TypeRef :: INT) , | _ | { FieldFuture :: new (async { Ok (Some (Value :: from (400))) }) })) ; let interface = Interface :: new ("MyInterface") . field (InterfaceField :: new ("a" , TypeRef :: named (TypeRef :: INT))) ; let query = Object :: new ("Query") . field (Field :: new ("valueA" , TypeRef :: named_nn (interface . type_name ()) , | _ | FieldFuture :: new (async { Ok (Some (FieldValue :: NULL . with_type ("MyObjA"))) }) ,)) . field (Field :: new ("valueB" , TypeRef :: named_nn (interface . type_name ()) , | _ | FieldFuture :: new (async { Ok (Some (FieldValue :: NULL . with_type ("MyObjB"))) }) ,)) ; let schema = Schema :: build (query . type_name () , None , None) . register (obj_a) . register (obj_b) . register (interface) . register (query) . finish () . unwrap () ; let query = r#"
        fragment A on MyObjA {
            b
        }

        fragment B on MyObjB {
            c
        }

        {
            valueA { __typename a ...A ...B }
            valueB { __typename a ...A ...B }
        }
        "# ; assert_eq ! (schema . execute (query) . await . into_result () . unwrap () . data , value ! ({ "valueA" : { "__typename" : "MyObjA" , "a" : 100 , "b" : 200 , } , "valueB" : { "__typename" : "MyObjB" , "a" : 300 , "c" : 400 , } })) ; } # [tokio :: test] async fn does_not_implement () { let obj_a = Object :: new ("MyObjA") . field (Field :: new ("a" , TypeRef :: named (TypeRef :: INT) , | _ | { FieldFuture :: new (async { Ok (Some (Value :: from (100))) }) })) . field (Field :: new ("b" , TypeRef :: named (TypeRef :: INT) , | _ | { FieldFuture :: new (async { Ok (Some (Value :: from (200))) }) })) ; let interface = Interface :: new ("MyInterface") . field (InterfaceField :: new ("a" , TypeRef :: named (TypeRef :: INT))) ; let query = Object :: new ("Query") . field (Field :: new ("valueA" , TypeRef :: named_nn (interface . type_name ()) , | _ | FieldFuture :: new (async { Ok (Some (FieldValue :: NULL . with_type ("MyObjA"))) }) ,)) ; let schema = Schema :: build (query . type_name () , None , None) . register (obj_a) . register (interface) . register (query) . finish () . unwrap () ; let query = r#"
        {
            valueA { a }
        }
        "# ; assert_eq ! (schema . execute (query) . await . into_result () . unwrap_err () , vec ! [ServerError { message : "internal: object \"MyObjA\" does not implement interface \"MyInterface\"" . to_owned () , source : None , locations : vec ! [Pos { column : 13 , line : 3 }] , path : vec ! [PathSegment :: Field ("valueA" . to_owned ())] , extensions : None , }]) ; } # [tokio :: test] async fn query_type_condition () { struct MyObjA ; let obj_a = Object :: new ("MyObjA") . implement ("MyInterface") . field (Field :: new ("a" , TypeRef :: named (TypeRef :: INT) , | _ | { FieldFuture :: new (async { Ok (Some (Value :: from (100))) }) })) . field (Field :: new ("b" , TypeRef :: named (TypeRef :: INT) , | _ | { FieldFuture :: new (async { Ok (Some (Value :: from (200))) }) })) ; let interface = Interface :: new ("MyInterface") . field (InterfaceField :: new ("a" , TypeRef :: named (TypeRef :: INT))) ; let query = Object :: new ("Query") ; let query = query . field (Field :: new ("valueA" , TypeRef :: named_nn (obj_a . type_name ()) , | _ | FieldFuture :: new (async { Ok (Some (FieldValue :: owned_any (MyObjA))) }) ,)) ; let schema = Schema :: build (query . type_name () , None , None) . register (obj_a) . register (interface) . register (query) . finish () . unwrap () ; let query = r#"
        {
            valueA { __typename
            b
            ... on MyInterface { a } }
        }
        "# ; assert_eq ! (schema . execute (query) . await . into_result () . unwrap () . data , value ! ({ "valueA" : { "__typename" : "MyObjA" , "b" : 200 , "a" : 100 , } })) ; } }
};
}
