// Generated macro for run_type_info_query (function)
macro_rules! Depcrate_executor_tests_introspection_enumsrun_type_info_query {
() => {
// Module: crate::executor_tests::introspection::enums
// Provides: {"run_type_info_query"}
// Dependencies: {}
async fn run_type_info_query < F > (doc : & str , f : F) where F : Fn ((& Object < DefaultScalarValue > , & Vec < Value < DefaultScalarValue > >)) , { let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let (result , errs) = crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; let type_info = result . as_object_value () . expect ("Result is not an object") . get_field_value ("__type") . expect ("__type field missing") . as_object_value () . expect ("__type field not an object value") ; let values = type_info . get_field_value ("enumValues") . expect ("enumValues field missing") . as_list_value () . expect ("enumValues not a list") ; f ((type_info , values)) ; }
};
}
