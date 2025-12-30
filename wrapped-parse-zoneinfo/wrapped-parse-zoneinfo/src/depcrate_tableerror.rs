// Generated macro for Error (enum)
macro_rules! Depcrate_tableError {
() => {
// Module: crate::table
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Something that can go wrong while constructing a `Table`."] # [derive (PartialEq , Debug , Copy , Clone)] pub enum Error < 'line > { # [doc = " A continuation line was passed in, but the previous line wasn’t a zone"] # [doc = " definition line."] SurpriseContinuationLine , # [doc = " A zone definition referred to a ruleset that hadn’t been defined."] UnknownRuleset (& 'line str) , # [doc = " A link line was passed in, but there’s already a link with that name."] DuplicateLink (& 'line str) , # [doc = " A zone line was passed in, but there’s already a zone with that name."] DuplicateZone , }
};
}
