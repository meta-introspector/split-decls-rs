// Generated macro for ALLOC (static)
macro_rules! DepcrateALLOC {
() => {
// Module: crate
// Provides: {"ALLOC"}
// Dependencies: {}
# [cfg (test)] # [global_allocator] static ALLOC : tikv_jemallocator :: Jemalloc = tikv_jemallocator :: Jemalloc ;
};
}
