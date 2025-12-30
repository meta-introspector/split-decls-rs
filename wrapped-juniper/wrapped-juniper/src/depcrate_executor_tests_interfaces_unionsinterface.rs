// Generated macro for interface (module)
macro_rules! Depcrate_executor_tests_interfaces_unionsinterface {
() => {
// Module: crate::executor_tests::interfaces_unions
// Provides: {"interface"}
// Dependencies: {}
mod interface { use crate :: { GraphQLObject , graphql_interface , graphql_object , schema :: model :: RootNode , types :: scalars :: { EmptyMutation , EmptySubscription } , } ; # [expect (dead_code , reason = "GraphQL schema testing")] # [graphql_interface] # [graphql (for = [Cat , Dog])] trait Pet { fn name (& self) -> & str ; } # [derive (GraphQLObject)] # [graphql (impl = PetValue)] struct Dog { name : String , woofs : bool , } # [derive (GraphQLObject)] # [graphql (impl = PetValue)] struct Cat { name : String , meows : bool , } struct Schema { pets : Vec < PetValue > , } # [graphql_object] impl Schema { fn pets (& self) -> & Vec < PetValue > { & self . pets } } # [tokio :: test] async fn test () { let schema = RootNode :: new (Schema { pets : vec ! [Dog { name : "Odie" . into () , woofs : true , } . into () , Cat { name : "Garfield" . into () , meows : false , } . into () ,] , } , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let doc = r"
          {
            pets {
              name
              ... on Dog {
                woofs
              }
              ... on Cat {
                meows
              }
            }
          }" ; let vars = vec ! [] . into_iter () . collect () ; let (result , errs) = crate :: execute (doc , None , & schema , & vars , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; assert_eq ! (result , graphql_value ! ({ "pets" : [{ "name" : "Odie" , "woofs" : true , } , { "name" : "Garfield" , "meows" : false , }] , }) ,) ; } }
};
}
