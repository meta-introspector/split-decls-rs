// Generated macro for Table (struct)
macro_rules! Depcrate_output_tableTable {
() => {
// Module: crate::output::table
// Provides: {"Table"}
// Dependencies: {}
pub struct Table < 'a > { columns : Vec < Column > , theme : & 'a Theme , env : & 'a Environment , widths : TableWidths , time_format : TimeFormat , size_format : SizeFormat , # [cfg (unix)] user_format : UserFormat , # [cfg (unix)] group_format : GroupFormat , flags_format : FlagsFormat , git : Option < & 'a GitCache > , }
};
}
