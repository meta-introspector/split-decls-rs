// Generated macro for TIMER_QUEUE_SIZE (const)
macro_rules! Depcrate_reactorTIMER_QUEUE_SIZE {
() => {
// Module: crate::reactor
// Provides: {"TIMER_QUEUE_SIZE"}
// Dependencies: {}
# [doc = " ESP-IDF - being an embedded OS - does not need so many timers"] # [doc = " and this saves ~ 20K RAM which is a lot for an MCU with RAM < 400K"] # [cfg (target_os = "espidf")] const TIMER_QUEUE_SIZE : usize = 100 ;
};
}
