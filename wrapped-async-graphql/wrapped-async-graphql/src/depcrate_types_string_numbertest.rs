// Generated macro for test (module)
macro_rules! Depcrate_types_string_numbertest {
() => {
// Module: crate::types::string_number
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: * ; # [tokio :: test] async fn test_string_number () { struct Query ; # [Object (internal)] impl Query { async fn value (& self , n : StringNumber < i32 >) -> StringNumber < i32 > { n } } let schema = Schema :: new (Query , EmptyMutation , EmptySubscription) ; assert_eq ! (schema . execute (r#"{
                    value1: value(n: "100")
                    value2: value(n: "-100")
                    value3: value(n: "0")
                    value4: value(n: "1")
                }"#) . await . into_result () . unwrap () . data , value ! ({ "value1" : "100" , "value2" : "-100" , "value3" : "0" , "value4" : "1" , })) ; } }
};
}
