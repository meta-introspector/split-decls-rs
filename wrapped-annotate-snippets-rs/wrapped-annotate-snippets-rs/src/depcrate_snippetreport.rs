// Generated macro for Report (type)
macro_rules! Depcrate_snippetReport {
() => {
// Module: crate::snippet
// Provides: {"Report"}
// Dependencies: {}
# [doc = " A [diagnostic message][Title] and any associated [context][Element] to help users"] # [doc = " understand it"] # [doc = ""] # [doc = " The first [`Group`] is the [\"primary\" group][Level::primary_title], ie it contains the diagnostic"] # [doc = " message."] # [doc = ""] # [doc = " All subsequent [`Group`]s are for distinct pieces of [context][Level::secondary_title]."] # [doc = " The primary group will be visually distinguished to help tell them apart."] pub type Report < 'a > = & 'a [Group < 'a >] ;
};
}
