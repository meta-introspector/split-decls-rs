// Generated macro for test_node (function)
macro_rules! Depcrate_tests_type_info_teststest_node {
() => {
// Module: crate::tests::type_info_tests
// Provides: {"test_node"}
// Dependencies: {}
# [test] fn test_node () { let doc = r#"
        {
            foo,
            bar,
            baz
        }"# ; let node_info = NodeTypeInfo { name : "MyNode" . into () , attribute_names : vec ! ["foo" . into () , "bar" . into () , "baz" . into ()] , } ; let mut node = Node { attributes : IndexMap :: new () , } ; node . attributes . insert ("foo" . into () , "1" . into ()) ; node . attributes . insert ("bar" . into () , "2" . into ()) ; node . attributes . insert ("baz" . into () , "3" . into ()) ; let schema : RootNode < _ , _ , _ > = RootNode :: new_with_info (node , EmptyMutation :: new () , EmptySubscription :: new () , node_info , () , () ,) ; assert_eq ! (crate :: execute_sync (doc , None , & schema , & graphql :: vars ! { } , & ()) , Ok ((graphql :: value ! ({ "foo" : "1" , "bar" : "2" , "baz" : "3" , }) , vec ! [] ,)) ,) ; }
};
}
