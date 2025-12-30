// Generated macro for ZoneRulesP (enum)
macro_rules! Depcrate_tz_zicZoneRulesP {
() => {
// Module: crate::tz::zic
// Provides: {"ZoneRulesP"}
// Dependencies: {}
# [doc = " The rule specification for a zone."] # [derive (Clone , Debug , Eq , PartialEq)] enum ZoneRulesP { # [doc = " No rules are used. Standard time always applies."] None , # [doc = " This zone uses a set of rules with the given name."] Named (RuleNameP) , # [doc = " An inline rule corresponding to a fixed offset from standard time for"] # [doc = " this zone."] Save (RuleSaveP) , }
};
}
