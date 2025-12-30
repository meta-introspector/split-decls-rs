// Generated macro for Track (struct)
macro_rules! Depcrate_allocTrack {
() => {
// Module: crate::alloc
// Provides: {"Track"}
// Dependencies: {}
# [doc = " Track allocations, detecting leaks"] # [derive (Debug)] pub struct Track < T > { value : T , # [doc = " Drop guard tracking the allocation's lifetime."] _obj : rt :: Allocation , }
};
}
