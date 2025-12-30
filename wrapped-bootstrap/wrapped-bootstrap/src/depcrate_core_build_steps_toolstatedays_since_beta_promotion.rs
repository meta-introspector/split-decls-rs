// Generated macro for days_since_beta_promotion (function)
macro_rules! Depcrate_core_build_steps_toolstatedays_since_beta_promotion {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"days_since_beta_promotion"}
// Dependencies: {}
# [doc = " Number of days after the last promotion of beta."] # [doc = " Its value is 41 on the Tuesday where \"Promote master to beta (T-2)\" happens."] # [doc = " The Wednesday after this has value 0."] # [doc = " We track this value to prevent regressing tools in the last week of the 6-week cycle."] fn days_since_beta_promotion () -> u64 { let since_epoch = t ! (time :: SystemTime :: UNIX_EPOCH . elapsed ()) ; (since_epoch . as_secs () / 86400 - 20) % 42 }
};
}
