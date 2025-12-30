// Generated macro for GlobalDataRaceHandler (enum)
macro_rules! Depcrate_concurrency_data_race_handlerGlobalDataRaceHandler {
() => {
// Module: crate::concurrency::data_race_handler
// Provides: {"GlobalDataRaceHandler"}
// Dependencies: {}
pub enum GlobalDataRaceHandler { # [doc = " No data race detection will be done."] None , # [doc = " State required to run in GenMC mode."] # [doc = " In this mode, the program will be executed repeatedly to explore different concurrent executions."] # [doc = " The `GenmcCtx` must persist across multiple executions, so it is behind an `Rc`."] # [doc = ""] # [doc = " The `GenmcCtx` has several methods with which to inform it about events like atomic memory accesses."] # [doc = " In GenMC mode, some functionality is taken over by GenMC:"] # [doc = " - Memory Allocation:    Allocated addresses need to be consistent across executions, which Miri's allocator doesn't guarantee"] # [doc = " - Scheduling:           To influence which concurrent execution we will explore next, GenMC takes over scheduling"] # [doc = " - Atomic operations:    GenMC will ensure that we explore all possible values that the memory model allows"] # [doc = "   an atomic operation to see at any specific point of the program."] Genmc (Rc < GenmcCtx >) , # [doc = " The default data race detector for Miri using vector clocks."] Vclocks (Box < data_race :: GlobalState >) , }
};
}
