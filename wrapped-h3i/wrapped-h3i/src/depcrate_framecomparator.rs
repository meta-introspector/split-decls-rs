// Generated macro for Comparator (enum)
macro_rules! Depcrate_frameComparator {
() => {
// Module: crate::frame
// Provides: {"Comparator"}
// Dependencies: {}
# [derive (Clone)] enum Comparator { Frame (H3iFrame) , # [doc = " Specifies how to compare an incoming [`H3iFrame`] with this"] # [doc = " [`CloseTriggerFrame`]. Typically, the validation attempts to fuzzy-match"] # [doc = " the [`CloseTriggerFrame`] against the incoming [`H3iFrame`], but there"] # [doc = " are times where other behavior is desired (for example, checking"] # [doc = " deserialized JSON payloads in a headers frame, or ensuring a random"] # [doc = " value matches a regex)."] # [doc = ""] # [doc = " See [`CloseTriggerFrame::is_equivalent`] for more on how frames are"] # [doc = " compared."] Fn (Arc < CustomEquivalenceHandler >) , }
};
}
