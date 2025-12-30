// Generated macro for UseMarkable (trait)
macro_rules! Depcrate_use_trackingUseMarkable {
() => {
// Module: crate::use_tracking
// Provides: {"UseMarkable"}
// Dependencies: {}
# [doc = " Models a thing that may have type variables in it that"] # [doc = " can be marked as 'used' as defined by `UseTracker`."] pub trait UseMarkable { fn mark_uses (& self , tracker : & mut UseTracker) ; }
};
}
