// Generated macro for Arc (struct)
macro_rules! Depcrate_util_syncArc {
() => {
// Module: crate::util::sync
// Provides: {"Arc"}
// Dependencies: {}
# [doc = " A \"fake\" `Arc`."] # [doc = ""] # [doc = " Basically, it exposes the `Arc` APIs we use in Jiff, but doesn't"] # [doc = " actually introduce indirection or reference counting. It's only used"] # [doc = " in core-only mode and in effect results in inlining all data into its"] # [doc = " container."] # [doc = ""] # [doc = " Not ideal, but we use `Arc` in very few places. One is `TimeZone`,"] # [doc = " which ends up being pretty small in core-only mode since it doesn't"] # [doc = " support carrying TZif data."] # [cfg (not (feature = "alloc"))] # [derive (Clone , Debug , Eq , PartialEq)] pub (crate) struct Arc < T > (T) ;
};
}
