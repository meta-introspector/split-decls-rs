// Generated macro for integration_test (module)
macro_rules! Depcrate_integrations_timeintegration_test {
() => {
// Module: crate::integrations::time
// Provides: {"integration_test"}
// Dependencies: {}
# [cfg (test)] mod integration_test { use time :: macros :: { date , datetime , offset , time } ; use crate :: { execute , graphql , graphql_object , schema :: model :: RootNode , types :: scalars :: { EmptyMutation , EmptySubscription } , } ; use super :: { DateTime , LocalDate , LocalDateTime , LocalTime , UtcOffset } ; # [tokio :: test] async fn serializes () { struct Root ; # [graphql_object] impl Root { fn local_date () -> LocalDate { date ! (2015 - 03 - 14) } fn local_time () -> LocalTime { time ! (16 : 07 : 08) } fn local_date_time () -> LocalDateTime { datetime ! (2016 - 07 - 08 09 : 10 : 11) } fn date_time () -> DateTime { datetime ! (1996 - 12 - 19 16 : 39 : 57 - 8) } fn utc_offset () -> UtcOffset { offset ! (+ 11 : 30) } } const DOC : & str = r#"{
            localDate
            localTime
            localDateTime
            dateTime,
            utcOffset,
        }"# ; let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; assert_eq ! (execute (DOC , None , & schema , & graphql :: vars ! { } , & ()) . await , Ok ((graphql :: value ! ({ "localDate" : "2015-03-14" , "localTime" : "16:07:08" , "localDateTime" : "2016-07-08T09:10:11" , "dateTime" : "1996-12-20T00:39:57Z" , "utcOffset" : "+11:30" , }) , vec ! [] ,)) ,) ; } }
};
}
