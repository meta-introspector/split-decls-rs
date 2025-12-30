// Generated macro for TestEvent (enum)
macro_rules! Depcrate_libtestTestEvent {
() => {
// Module: crate::libtest
// Provides: {"TestEvent"}
// Dependencies: {}
# [derive (Debug , PartialEq , Deserialize , Serialize)] # [serde (tag = "event")] # [serde (rename_all = "lowercase")] # [doc = " Test event"] pub enum TestEvent { # [doc = " a new test starts"] Started { # [doc = " the name of this test"] name : String , } , # [doc = " the test has finished"] Ok { # [doc = " which one"] name : String , # [doc = " in how long"] exec_time : f32 , # [doc = " what did it say?"] stdout : Option < String > , } , # [doc = " the test has failed"] Failed { # [doc = " which one"] name : String , # [doc = " in how long"] exec_time : f32 , # [doc = " why?"] stdout : Option < String > , # [doc = " it timed out?"] reason : Option < String > , # [doc = " what message"] message : Option < String > , } , # [doc = " the test has been ignored"] Ignored { # [doc = " which one"] name : String , } , # [doc = " the test has timed out"] Timeout { # [doc = " which one"] name : String , } , }
};
}
