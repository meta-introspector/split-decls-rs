// Generated macro for impl_840 (impl)
macro_rules! Depcrate_output_tableimpl_840 {
() => {
// Module: crate::output::table
// Provides: {"impl_840"}
// Dependencies: {}
impl TimeType { # [doc = " Returns the text to use for a column’s heading in the columns output."] pub fn header (self) -> & 'static str { match self { Self :: Modified => "Date Modified" , Self :: Changed => "Date Changed" , Self :: Accessed => "Date Accessed" , Self :: Created => "Date Created" , } } # [doc = " Returns the corresponding time from [File]"] pub fn get_corresponding_time (self , file : & File < '_ >) -> Option < NaiveDateTime > { match self { TimeType :: Modified => file . modified_time () , TimeType :: Changed => file . changed_time () , TimeType :: Accessed => file . accessed_time () , TimeType :: Created => file . created_time () , } } }
};
}
