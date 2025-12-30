// Generated macro for run_type_info_query (function)
macro_rules! Depcrate_executor_tests_introspection_input_objectrun_type_info_query {
() => {
// Module: crate::executor_tests::introspection::input_object
// Provides: {"run_type_info_query"}
// Dependencies: {}
async fn run_type_info_query < F > (doc : & str , f : F) where F : Fn (& Object < DefaultScalarValue > , & Vec < Value < DefaultScalarValue > >) , { let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let (result , errs) = crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & ()) . await . expect ("Execution failed") ; assert_eq ! (errs , []) ; println ! ("Result: {result:#?}") ; let type_info = result . as_object_value () . expect ("Result is not an object") . get_field_value ("__type") . expect ("__type field missing") . as_object_value () . expect ("__type field not an object value") ; let fields = type_info . get_field_value ("inputFields") . expect ("inputFields field missing") . as_list_value () . expect ("inputFields not a list") ; f (type_info , fields) ; }
};
}
