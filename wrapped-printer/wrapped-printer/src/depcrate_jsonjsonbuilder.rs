// Generated macro for JSONBuilder (struct)
macro_rules! Depcrate_jsonJSONBuilder {
() => {
// Module: crate::json
// Provides: {"JSONBuilder"}
// Dependencies: {}
# [doc = " A builder for a JSON lines printer."] # [doc = ""] # [doc = " The builder permits configuring how the printer behaves. The JSON printer"] # [doc = " has fewer configuration options than the standard printer because it is"] # [doc = " a structured format, and the printer always attempts to find the most"] # [doc = " information possible."] # [doc = ""] # [doc = " Some configuration options, such as whether line numbers are included or"] # [doc = " whether contextual lines are shown, are drawn directly from the"] # [doc = " `grep_searcher::Searcher`'s configuration."] # [doc = ""] # [doc = " Once a `JSON` printer is built, its configuration cannot be changed."] # [derive (Clone , Debug)] pub struct JSONBuilder { config : Config , }
};
}
