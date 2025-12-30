// Generated macro for MIN_DATE (const)
macro_rules! Depcrate_dateMIN_DATE {
() => {
// Module: crate::date
// Provides: {"MIN_DATE"}
// Dependencies: {}
# [doc = " The minimum possible `Date`."] # [allow (deprecated)] # [deprecated (since = "0.4.20" , note = "Use Date::MIN_UTC instead")] pub const MIN_DATE : Date < Utc > = Date :: < Utc > :: MIN_UTC ;
};
}
