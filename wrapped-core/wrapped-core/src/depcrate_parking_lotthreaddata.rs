// Generated macro for ThreadData (struct)
macro_rules! Depcrate_parking_lotThreadData {
() => {
// Module: crate::parking_lot
// Provides: {"ThreadData"}
// Dependencies: {}
struct ThreadData { parker : ThreadParker , key : AtomicUsize , next_in_queue : Cell < * const ThreadData > , unpark_token : Cell < UnparkToken > , park_token : Cell < ParkToken > , parked_with_timeout : Cell < bool > , # [cfg (feature = "deadlock_detection")] deadlock_data : deadlock :: DeadlockData , }
};
}
