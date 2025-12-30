// Generated macro for Table (struct)
macro_rules! Depcrate_tableTable {
() => {
// Module: crate::table
// Provides: {"Table"}
// Dependencies: {}
# [doc = " A **table** of all the data in one or more zoneinfo files."] # [derive (PartialEq , Debug , Default)] pub struct Table { # [doc = " Mapping of ruleset names to rulesets."] pub rulesets : HashMap < String , Vec < RuleInfo > > , # [doc = " Mapping of zoneset names to zonesets."] pub zonesets : HashMap < String , Vec < ZoneInfo > > , # [doc = " Mapping of link timezone names, to the names they link to."] pub links : HashMap < String , String > , }
};
}
