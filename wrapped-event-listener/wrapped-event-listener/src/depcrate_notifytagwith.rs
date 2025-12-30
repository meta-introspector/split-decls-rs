// Generated macro for TagWith (struct)
macro_rules! Depcrate_notifyTagWith {
() => {
// Module: crate::notify
// Provides: {"TagWith"}
// Dependencies: {}
# [doc = " Use a function to generate a tag to notify listeners."] # [cfg (feature = "std")] # [doc (hidden)] pub struct TagWith < N : ? Sized , F > { tag : F , inner : N , }
};
}
