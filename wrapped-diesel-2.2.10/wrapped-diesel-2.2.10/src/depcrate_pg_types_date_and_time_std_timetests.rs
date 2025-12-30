// Generated macro for tests (module)
macro_rules! Depcrate_pg_types_date_and_time_std_timetests {
() => {
// Module: crate::pg::types::date_and_time::std_time
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { extern crate dotenvy ; use std :: time :: { Duration , SystemTime , UNIX_EPOCH } ; use crate :: dsl :: { now , sql } ; use crate :: prelude :: * ; use crate :: select ; use crate :: sql_types :: Timestamp ; use crate :: test_helpers :: pg_connection ; # [test] fn unix_epoch_encodes_correctly () { let connection = & mut pg_connection () ; let query = select (sql :: < Timestamp > ("'1970-01-01'") . eq (UNIX_EPOCH)) ; assert ! (query . get_result ::< bool > (connection) . unwrap ()) ; } # [test] fn unix_epoch_decodes_correctly () { let connection = & mut pg_connection () ; let epoch_from_sql = select (sql :: < Timestamp > ("'1970-01-01'::timestamp")) . get_result :: < SystemTime > (connection) ; assert_eq ! (Ok (UNIX_EPOCH) , epoch_from_sql) ; } # [test] fn times_relative_to_now_encode_correctly () { let connection = & mut pg_connection () ; let time = SystemTime :: now () + Duration :: from_secs (60) ; let query = select (now . at_time_zone ("utc") . lt (time)) ; assert ! (query . get_result ::< bool > (connection) . unwrap ()) ; let time = SystemTime :: now () - Duration :: from_secs (60) ; let query = select (now . at_time_zone ("utc") . gt (time)) ; assert ! (query . get_result ::< bool > (connection) . unwrap ()) ; } }
};
}
