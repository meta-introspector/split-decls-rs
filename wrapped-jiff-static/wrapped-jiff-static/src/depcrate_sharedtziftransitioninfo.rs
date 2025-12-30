// Generated macro for TzifTransitionInfo (struct)
macro_rules! Depcrate_sharedTzifTransitionInfo {
() => {
// Module: crate::shared
// Provides: {"TzifTransitionInfo"}
// Dependencies: {}
# [doc = " TZif transition info beyond the timestamp and civil datetime."] # [doc = ""] # [doc = " For example, this contains a transition's \"local type index,\" which in"] # [doc = " turn gives access to the offset (among other metadata) for that transition."] # [derive (Clone , Copy , Debug)] pub struct TzifTransitionInfo { # [doc = " The index into the sequence of local time type records. This is what"] # [doc = " provides the correct offset (from UTC) that is active beginning at"] # [doc = " this transition."] pub type_index : u8 , # [doc = " The boundary condition for quickly determining if a given wall clock"] # [doc = " time is ambiguous (i.e., falls in a gap or a fold)."] pub kind : TzifTransitionKind , }
};
}
