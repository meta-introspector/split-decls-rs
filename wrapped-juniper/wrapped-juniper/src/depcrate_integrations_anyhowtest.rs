// Generated macro for test (module)
macro_rules! Depcrate_integrations_anyhowtest {
() => {
// Module: crate::integrations::anyhow
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use std :: env ; use anyhow :: anyhow ; use serial_test :: serial ; use crate :: { EmptyMutation , EmptySubscription , RootNode , execute , graphql , graphql_object , parser :: SourcePosition , } ; # [tokio :: test] # [serial] async fn simple () { struct Root ; # [graphql_object] impl Root { fn err () -> anyhow :: Result < i32 > { Err (anyhow ! ("errored!")) } } let prev_env = env :: var ("RUST_BACKTRACE") . ok () ; unsafe { env :: set_var ("RUST_BACKTRACE" , "1") ; } const DOC : & str = r#"{
            err
        }"# ; let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let res = execute (DOC , None , & schema , & graphql :: vars ! { } , & ()) . await ; assert ! (res . is_ok () , "failed: {:?}" , res . unwrap_err ()) ; let (val , errs) = res . unwrap () ; assert_eq ! (val , graphql :: value ! (null)) ; assert_eq ! (errs . len () , 1 , "too many errors: {errs:?}") ; let err = errs . first () . unwrap () ; assert_eq ! (* err . location () , SourcePosition :: new (14 , 1 , 12)) ; assert_eq ! (err . path () , & ["err"]) ; let err = err . error () ; assert_eq ! (err . message () , "errored!") ; # [cfg (not (any (nightly , feature = "backtrace")))] assert_eq ! (err . extensions () , & graphql :: value ! (null)) ; # [cfg (any (nightly , feature = "backtrace"))] assert_eq ! (err . extensions () . as_object_value () . map (| ext | ext . contains_field ("backtrace")) , Some (true) , "no `backtrace` in extensions: {err:?}" ,) ; if let Some (val) = prev_env { unsafe { env :: set_var ("RUST_BACKTRACE" , val) ; } } } }
};
}
