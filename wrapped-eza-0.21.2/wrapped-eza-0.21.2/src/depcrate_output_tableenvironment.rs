// Generated macro for Environment (struct)
macro_rules! Depcrate_output_tableEnvironment {
() => {
// Module: crate::output::table
// Provides: {"Environment"}
// Dependencies: {}
# [doc = " The **environment** struct contains any data that could change between"] # [doc = " running instances of exa, depending on the user’s computer’s configuration."] # [doc = ""] # [doc = " Any environment field should be able to be mocked up for test runs."] pub struct Environment { # [doc = " The computer’s current time offset, determined from time zone."] time_offset : FixedOffset , # [doc = " Localisation rules for formatting numbers."] numeric : locale :: Numeric , # [doc = " Mapping cache of user IDs to usernames."] # [cfg (unix)] users : Mutex < UsersCache > , }
};
}
