// Generated macro for DefaultCustom (struct)
macro_rules! Depcrate_fmt_strtimeDefaultCustom {
() => {
// Module: crate::fmt::strtime
// Provides: {"DefaultCustom"}
// Dependencies: {}
# [doc = " The default trait implementation of [`Custom`]."] # [doc = ""] # [doc = " Whenever one uses the formatting or parsing routines in this module"] # [doc = " without providing a configuration, then this customization is the one"] # [doc = " that gets used."] # [doc = ""] # [doc = " The behavior of the locale formatting of this type is meant to match that"] # [doc = " of Unicode's `und` locale."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to explicitly use [`DefaultCustom`] via `strtime`"] # [doc = " formatting:"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::{civil, fmt::strtime::{BrokenDownTime, DefaultCustom, Config}};"] # [doc = ""] # [doc = " let config = Config::new().custom(DefaultCustom::new());"] # [doc = " let dt = civil::date(2025, 7, 1).at(17, 30, 0, 0);"] # [doc = " let tm = BrokenDownTime::from(dt);"] # [doc = " assert_eq!("] # [doc = "     tm.to_string_with_config(&config, \"%c\")?,"] # [doc = "     \"2025 M07 1, Tue 17:30:00\","] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Debug , Default)] pub struct DefaultCustom (()) ;
};
}
