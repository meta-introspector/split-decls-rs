// Generated macro for Args (struct)
macro_rules! DepcrateArgs {
() => {
// Module: crate
// Provides: {"Args"}
// Dependencies: {}
# [derive (Parser)] # [command (author = "The ICU4X Project Developers" , about = "Download data from CLDR and ICU for ICU4X testing")] struct Args { # [arg (short , long , help = "Sets the level of verbosity (-v, -vv, or -vvv)" , action = ArgAction :: Count)] verbose : u8 , }
};
}
