// Generated macro for SummaryBuilder (struct)
macro_rules! Depcrate_summarySummaryBuilder {
() => {
// Module: crate::summary
// Provides: {"SummaryBuilder"}
// Dependencies: {}
# [doc = " A builder for summary printer."] # [doc = ""] # [doc = " The builder permits configuring how the printer behaves. The summary"] # [doc = " printer has fewer configuration options than the standard printer because"] # [doc = " it aims to produce aggregate output about a single search (typically just"] # [doc = " one line) instead of output for each match."] # [doc = ""] # [doc = " Once a `Summary` printer is built, its configuration cannot be changed."] # [derive (Clone , Debug)] pub struct SummaryBuilder { config : Config , }
};
}
