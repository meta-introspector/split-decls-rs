// Generated macro for GroupInfoInner (struct)
macro_rules! Depcrate_util_capturesGroupInfoInner {
() => {
// Module: crate::util::captures
// Provides: {"GroupInfoInner"}
// Dependencies: {}
# [doc = " The inner guts of `GroupInfo`. This type only exists so that it can"] # [doc = " be wrapped in an `Arc` to make `GroupInfo` reference counted."] # [derive (Debug , Default)] struct GroupInfoInner { slot_ranges : Vec < (SmallIndex , SmallIndex) > , name_to_index : Vec < CaptureNameMap > , index_to_name : Vec < Vec < Option < Arc < str > > > > , memory_extra : usize , }
};
}
