// Generated macro for IoAdapter (struct)
macro_rules! Depcrate_integrations_timeIoAdapter {
() => {
// Module: crate::integrations::time
// Provides: {"IoAdapter"}
// Dependencies: {}
# [doc = " [`io::Write`] adapter for [`fmt::Formatter`]."] # [doc = ""] # [doc = " Required because [`time`] crate cannot write to [`fmt::Write`], only to [`io::Write`]."] struct IoAdapter < 'a , 'b > (& 'a mut fmt :: Formatter < 'b >) ;
};
}
