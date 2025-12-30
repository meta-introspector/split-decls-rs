// Generated macro for State (enum)
macro_rules! Depcrate_events_attributesState {
() => {
// Module: crate::events::attributes
// Provides: {"State"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] enum State { # [doc = " Iteration finished, iterator will return `None` to all [`IterState::next`]"] # [doc = " requests."] Done , # [doc = " The last attribute returned was deserialized successfully. Contains an"] # [doc = " offset from which next attribute should be searched."] Next (usize) , # [doc = " The last attribute returns [`AttrError::UnquotedValue`], offset pointed"] # [doc = " to the beginning of the value. Recover should skip a value"] SkipValue (usize) , # [doc = " The last attribute returns [`AttrError::Duplicated`], offset pointed to"] # [doc = " the equal (`=`) sign. Recover should skip it and a value"] SkipEqValue (usize) , }
};
}
