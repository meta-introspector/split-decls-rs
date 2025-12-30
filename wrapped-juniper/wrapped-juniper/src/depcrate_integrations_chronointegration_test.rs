// Generated macro for integration_test (module)
macro_rules! Depcrate_integrations_chronointegration_test {
() => {
// Module: crate::integrations::chrono
// Provides: {"integration_test"}
// Dependencies: {}
# [cfg (test)] mod integration_test { use crate :: { execute , graphql , graphql_object , schema :: model :: RootNode , types :: scalars :: { EmptyMutation , EmptySubscription } , } ; use super :: { DateTime , FixedOffset , FromFixedOffset , LocalDate , LocalDateTime , LocalTime , TimeZone , } ; # [tokio :: test] async fn serializes () { # [derive (Clone , Copy)] struct CET ; impl TimeZone for CET { type Offset = < chrono_tz :: Tz as TimeZone > :: Offset ; fn from_offset (_ : & Self :: Offset) -> Self { CET } fn offset_from_local_date (& self , local : & chrono :: NaiveDate ,) -> chrono :: LocalResult < Self :: Offset > { chrono_tz :: CET . offset_from_local_date (local) } fn offset_from_local_datetime (& self , local : & chrono :: NaiveDateTime ,) -> chrono :: LocalResult < Self :: Offset > { chrono_tz :: CET . offset_from_local_datetime (local) } fn offset_from_utc_date (& self , utc : & chrono :: NaiveDate) -> Self :: Offset { chrono_tz :: CET . offset_from_utc_date (utc) } fn offset_from_utc_datetime (& self , utc : & chrono :: NaiveDateTime) -> Self :: Offset { chrono_tz :: CET . offset_from_utc_datetime (utc) } } impl FromFixedOffset for CET { fn from_fixed_offset (dt : DateTime < FixedOffset >) -> DateTime < Self > { dt . with_timezone (& CET) } } struct Root ; # [graphql_object] impl Root { fn local_date () -> LocalDate { LocalDate :: from_ymd_opt (2015 , 3 , 14) . unwrap () } fn local_time () -> LocalTime { LocalTime :: from_hms_opt (16 , 7 , 8) . unwrap () } fn local_date_time () -> LocalDateTime { LocalDateTime :: new (LocalDate :: from_ymd_opt (2016 , 7 , 8) . unwrap () , LocalTime :: from_hms_opt (9 , 10 , 11) . unwrap () ,) } fn date_time () -> DateTime < chrono :: Utc > { DateTime :: from_naive_utc_and_offset (LocalDateTime :: new (LocalDate :: from_ymd_opt (1996 , 12 , 20) . unwrap () , LocalTime :: from_hms_opt (0 , 39 , 57) . unwrap () ,) , chrono :: Utc ,) } fn pass_date_time (dt : DateTime < CET >) -> DateTime < CET > { dt } fn transform_date_time (dt : DateTime < CET >) -> DateTime < chrono :: Utc > { dt . with_timezone (& chrono :: Utc) } } const DOC : & str = r#"{
            localDate
            localTime
            localDateTime
            dateTime,
            passDateTime(dt: "2014-11-28T21:00:09+09:00")
            transformDateTime(dt: "2014-11-28T21:00:09+09:00")
        }"# ; let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; assert_eq ! (execute (DOC , None , & schema , & graphql :: vars ! { } , & ()) . await , Ok ((graphql :: value ! ({ "localDate" : "2015-03-14" , "localTime" : "16:07:08" , "localDateTime" : "2016-07-08T09:10:11" , "dateTime" : "1996-12-20T00:39:57Z" , "passDateTime" : "2014-11-28T12:00:09Z" , "transformDateTime" : "2014-11-28T12:00:09Z" , }) , vec ! [] ,)) ,) ; } }
};
}
