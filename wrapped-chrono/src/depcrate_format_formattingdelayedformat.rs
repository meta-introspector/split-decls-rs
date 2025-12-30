// Generated macro for DelayedFormat (struct)
macro_rules! Depcrate_format_formattingDelayedFormat {
() => {
// Module: crate::format::formatting
// Provides: {"DelayedFormat"}
// Dependencies: {}
# [doc = " A *temporary* object which can be used as an argument to `format!` or others."] # [doc = " This is normally constructed via `format` methods of each date and time type."] # [cfg (feature = "alloc")] # [derive (Debug)] pub struct DelayedFormat < I > { # [doc = " The date view, if any."] date : Option < NaiveDate > , # [doc = " The time view, if any."] time : Option < NaiveTime > , # [doc = " The name and local-to-UTC difference for the offset (timezone), if any."] off : Option < (String , FixedOffset) > , # [doc = " An iterator returning formatting items."] items : I , # [doc = " Locale used for text."] # [doc = " ZST if the `unstable-locales` feature is not enabled."] locale : Locale , }
};
}
