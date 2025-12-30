// Generated macro for format (function)
macro_rules! Depcrate_format_formattingformat {
() => {
// Module: crate::format::formatting
// Provides: {"format"}
// Dependencies: {}
# [doc = " Tries to format given arguments with given formatting items."] # [doc = " Internally used by `DelayedFormat`."] # [cfg (feature = "alloc")] # [deprecated (since = "0.4.32" , note = "Use DelayedFormat::fmt or DelayedFormat::write_to instead")] pub fn format < 'a , I , B > (w : & mut fmt :: Formatter , date : Option < & NaiveDate > , time : Option < & NaiveTime > , off : Option < & (String , FixedOffset) > , items : I ,) -> fmt :: Result where I : Iterator < Item = B > + Clone , B : Borrow < Item < 'a > > , { DelayedFormat { date : date . copied () , time : time . copied () , off : off . cloned () , items , locale : default_locale () , } . fmt (w) }
};
}
