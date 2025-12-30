// Generated macro for timezones (module)
macro_rules! Depcratetimezones {
() => {
// Module: crate
// Provides: {"timezones"}
// Dependencies: {}
# [cfg (any (feature = "case-insensitive" , feature = "filter-by-regex"))] mod timezones { # ! [allow (non_camel_case_types , clippy :: unreadable_literal)] include ! (concat ! (env ! ("OUT_DIR") , "/timezones.rs")) ; }
};
}
