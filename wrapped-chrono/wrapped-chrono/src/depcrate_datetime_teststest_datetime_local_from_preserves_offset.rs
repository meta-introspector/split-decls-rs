// Generated macro for test_datetime_local_from_preserves_offset (function)
macro_rules! Depcrate_datetime_teststest_datetime_local_from_preserves_offset {
() => {
// Module: crate::datetime::tests
// Provides: {"test_datetime_local_from_preserves_offset"}
// Dependencies: {}
# [test] # [cfg (feature = "clock")] fn test_datetime_local_from_preserves_offset () { let naivedatetime = NaiveDate :: from_ymd_opt (2023 , 1 , 1) . unwrap () . and_hms_opt (0 , 0 , 0) . unwrap () ; let datetime = Local . from_utc_datetime (& naivedatetime) ; let offset = datetime . offset () . fix () ; let datetime_fixed : DateTime < FixedOffset > = datetime . into () ; assert_eq ! (& offset , datetime_fixed . offset ()) ; assert_eq ! (datetime . fixed_offset () , datetime_fixed) ; }
};
}
