// Generated macro for AllocDataRaceHandler (enum)
macro_rules! Depcrate_concurrency_data_race_handlerAllocDataRaceHandler {
() => {
// Module: crate::concurrency::data_race_handler
// Provides: {"AllocDataRaceHandler"}
// Dependencies: {}
# [derive (Debug)] pub enum AllocDataRaceHandler { None , Genmc , # [doc = " Data race detection via the use of vector clocks."] # [doc = " Weak memory emulation via the use of store buffers (if enabled)."] Vclocks (data_race :: AllocState , Option < weak_memory :: AllocState >) , }
};
}
