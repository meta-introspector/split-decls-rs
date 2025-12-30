// Generated macro for PosixCustom (struct)
macro_rules! Depcrate_fmt_strtimePosixCustom {
() => {
// Module: crate::fmt::strtime
// Provides: {"PosixCustom"}
// Dependencies: {}
# [doc = " A POSIX locale implementation of [`Custom`]."] # [doc = ""] # [doc = " The behavior of the locale formatting of this type is meant to match that"] # [doc = " of POSIX's `C` locale."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to use [`PosixCustom`] via `strtime` formatting:"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::{civil, fmt::strtime::{BrokenDownTime, PosixCustom, Config}};"] # [doc = ""] # [doc = " let config = Config::new().custom(PosixCustom::new());"] # [doc = " let dt = civil::date(2025, 7, 1).at(17, 30, 0, 0);"] # [doc = " let tm = BrokenDownTime::from(dt);"] # [doc = " assert_eq!("] # [doc = "     tm.to_string_with_config(&config, \"%c\")?,"] # [doc = "     \"Tue Jul  1 17:30:00 2025\","] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Debug , Default)] pub struct PosixCustom (()) ;
};
}
