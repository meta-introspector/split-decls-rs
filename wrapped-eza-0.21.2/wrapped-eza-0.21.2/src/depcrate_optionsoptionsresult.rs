// Generated macro for OptionsResult (enum)
macro_rules! Depcrate_optionsOptionsResult {
() => {
// Module: crate::options
// Provides: {"OptionsResult"}
// Dependencies: {}
# [doc = " The result of the `Options::parse` function."] # [doc = ""] # [doc = " NOTE: We disallow the `large_enum_variant` lint here, because we're not"] # [doc = " overly concerned about variant fragmentation. We can do this because we are"] # [doc = " reasonably sure that the error variant will be rare, and only on faulty"] # [doc = " program execution and thus boxing the large variant will be a waste of"] # [doc = " resources, but should we come to use it more, we should reconsider."] # [doc = ""] # [doc = " See <https://github.com/eza-community/eza/pull/437#issuecomment-1738470254>"] # [allow (clippy :: large_enum_variant)] # [derive (Debug)] pub enum OptionsResult < 'args > { # [doc = " The options were parsed successfully."] Ok (Options , Vec < & 'args OsStr >) , # [doc = " There was an error parsing the arguments."] InvalidOptions (OptionsError) , # [doc = " One of the arguments was `--help`, so display help."] Help (HelpString) , # [doc = " One of the arguments was `--version`, so display the version number."] Version (VersionString) , }
};
}
