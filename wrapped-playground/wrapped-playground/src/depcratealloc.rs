// Generated macro for ALLOC (static)
macro_rules! DepcrateALLOC {
() => {
// Module: crate
// Provides: {"ALLOC"}
// Dependencies: {}
# [cfg (feature = "wee_alloc")] # [global_allocator] static ALLOC : wee_alloc :: WeeAlloc = wee_alloc :: WeeAlloc :: INIT ;
};
}
