// Generated macro for TableBuilder (struct)
macro_rules! Depcrate_tableTableBuilder {
() => {
// Module: crate::table
// Provides: {"TableBuilder"}
// Dependencies: {}
# [doc = " A builder for `Table` values based on various line definitions."] # [derive (PartialEq , Debug)] pub struct TableBuilder { # [doc = " The table that’s being built up."] table : Table , # [doc = " If the last line was a zone definition, then this holds its name."] # [doc = " `None` otherwise. This is so continuation lines can be added to the"] # [doc = " same zone as the original zone line."] current_zoneset_name : Option < String > , }
};
}
