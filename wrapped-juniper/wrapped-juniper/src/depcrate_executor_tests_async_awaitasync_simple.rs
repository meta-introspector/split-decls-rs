// Generated macro for async_simple (function)
macro_rules! Depcrate_executor_tests_async_awaitasync_simple {
() => {
// Module: crate::executor_tests::async_await
// Provides: {"async_simple"}
// Dependencies: {}
# [tokio :: test] async fn async_simple () { let schema = RootNode :: new (Query , EmptyMutation :: new () , EmptySubscription :: new ()) ; let doc = r#"
        query {
            fieldSync
            fieldAsyncPlain
            delayed
            user(id: "user1") {
                name
            }
        }
    "# ; let (res , errs) = crate :: execute (doc , None , & schema , & graphql :: vars ! { } , & ()) . await . unwrap () ; assert ! (errs . is_empty ()) ; let obj = res . into_object () . unwrap () ; let value = Value :: Object (obj) ; assert_eq ! (value , graphql :: value ! ({ "delayed" : true , "fieldAsyncPlain" : "field_async_plain" , "fieldSync" : "field_sync" , "user" : { "name" : "user1" , } , }) ,) ; }
};
}
