// Generated macro for format_item (function)
macro_rules! Depcrate_format_formattingformat_item {
() => {
// Module: crate::format::formatting
// Provides: {"format_item"}
// Dependencies: {}
# [doc = " Formats single formatting item"] # [cfg (feature = "alloc")] # [deprecated (since = "0.4.32" , note = "Use DelayedFormat::fmt or DelayedFormat::write_to instead")] pub fn format_item (w : & mut fmt :: Formatter , date : Option < & NaiveDate > , time : Option < & NaiveTime > , off : Option < & (String , FixedOffset) > , item : & Item < '_ > ,) -> fmt :: Result { DelayedFormat { date : date . copied () , time : time . copied () , off : off . cloned () , items : [item] . into_iter () , locale : default_locale () , } . fmt (w) }
};
}
