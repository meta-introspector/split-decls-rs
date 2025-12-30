// Generated macro for StandardBuilder (struct)
macro_rules! Depcrate_standardStandardBuilder {
() => {
// Module: crate::standard
// Provides: {"StandardBuilder"}
// Dependencies: {}
# [doc = " A builder for the \"standard\" grep-like printer."] # [doc = ""] # [doc = " The builder permits configuring how the printer behaves. Configurable"] # [doc = " behavior includes, but is not limited to, limiting the number of matches,"] # [doc = " tweaking separators, executing pattern replacements, recording statistics"] # [doc = " and setting colors."] # [doc = ""] # [doc = " Some configuration options, such as the display of line numbers or"] # [doc = " contextual lines, are drawn directly from the"] # [doc = " `grep_searcher::Searcher`'s configuration."] # [doc = ""] # [doc = " Once a `Standard` printer is built, its configuration cannot be changed."] # [derive (Clone , Debug)] pub struct StandardBuilder { config : Config , }
};
}
