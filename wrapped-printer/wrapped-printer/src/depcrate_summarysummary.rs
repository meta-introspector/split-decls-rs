// Generated macro for Summary (struct)
macro_rules! Depcrate_summarySummary {
() => {
// Module: crate::summary
// Provides: {"Summary"}
// Dependencies: {}
# [doc = " The summary printer, which emits aggregate results from a search."] # [doc = ""] # [doc = " Aggregate results generally correspond to file paths and/or the number of"] # [doc = " matches found."] # [doc = ""] # [doc = " A default printer can be created with either of the `Summary::new` or"] # [doc = " `Summary::new_no_color` constructors. However, there are a number of"] # [doc = " options that configure this printer's output. Those options can be"] # [doc = " configured using [`SummaryBuilder`]."] # [doc = ""] # [doc = " This type is generic over `W`, which represents any implementation of"] # [doc = " the `termcolor::WriteColor` trait."] # [derive (Clone , Debug)] pub struct Summary < W > { config : Config , wtr : RefCell < CounterWriter < W > > , }
};
}
