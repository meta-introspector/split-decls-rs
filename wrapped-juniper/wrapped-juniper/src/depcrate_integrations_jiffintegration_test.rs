// Generated macro for integration_test (module)
macro_rules! Depcrate_integrations_jiffintegration_test {
() => {
// Module: crate::integrations::jiff
// Provides: {"integration_test"}
// Dependencies: {}
# [cfg (test)] mod integration_test { use jiff :: { ToSpan as _ , civil , tz } ; use crate :: { execute , graphql , graphql_object , schema :: model :: RootNode , types :: scalars :: { EmptyMutation , EmptySubscription } , } ; use super :: { DateTime , Duration , LocalDate , LocalDateTime , LocalTime , TimeZone , UtcOffset , ZonedDateTime , } ; # [tokio :: test] async fn serializes () { struct Root ; # [graphql_object] impl Root { fn local_date () -> LocalDate { LocalDate :: constant (2015 , 3 , 14) } fn local_time () -> LocalTime { LocalTime :: constant (16 , 7 , 8 , 0) } fn local_date_time () -> LocalDateTime { LocalDateTime :: constant (2016 , 7 , 8 , 9 , 10 , 11 , 0) } fn date_time () -> DateTime { civil :: DateTime :: constant (2014 , 11 , 28 , 12 , 0 , 9 , 50_000_000) . to_zoned (tz :: TimeZone :: UTC) . unwrap () . timestamp () } fn zoned_date_time () -> ZonedDateTime { civil :: DateTime :: constant (2014 , 11 , 28 , 12 , 0 , 9 , 50_000_000) . to_zoned (tz :: TimeZone :: get ("America/New_York") . unwrap ()) . unwrap () } fn time_zone () -> TimeZone { tz :: TimeZone :: get ("Asia/Tokyo") . unwrap () . try_into () . unwrap () } fn utc_offset () -> UtcOffset { tz :: offset (10) } fn duration () -> Duration { 1 . year () . months (1) . days (1) . hours (1) . minutes (1) . seconds (1) . milliseconds (100) } } const DOC : & str = r#"{
            localDate
            localTime
            localDateTime
            dateTime,
            zonedDateTime,
            timeZone,
            utcOffset,
            duration,
        }"# ; let schema = RootNode :: new (Root , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; assert_eq ! (execute (DOC , None , & schema , & graphql :: vars ! { } , & ()) . await , Ok ((graphql :: value ! ({ "localDate" : "2015-03-14" , "localTime" : "16:07:08" , "localDateTime" : "2016-07-08T09:10:11" , "dateTime" : "2014-11-28T12:00:09.05Z" , "zonedDateTime" : "2014-11-28T12:00:09.05-05:00[America/New_York]" , "timeZone" : "Asia/Tokyo" , "utcOffset" : "+10:00" , "duration" : "P1Y1M1DT1H1M1.1S" , }) , vec ! [] ,)) ,) ; } }
};
}
