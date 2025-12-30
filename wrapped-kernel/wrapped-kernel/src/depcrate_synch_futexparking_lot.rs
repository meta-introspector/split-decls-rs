// Generated macro for PARKING_LOT (static)
macro_rules! Depcrate_synch_futexPARKING_LOT {
() => {
// Module: crate::synch::futex
// Provides: {"PARKING_LOT"}
// Dependencies: {}
static PARKING_LOT : InterruptTicketMutex < HashMap < usize , TaskHandlePriorityQueue , RandomState > > = InterruptTicketMutex :: new (HashMap :: with_hasher (RandomState :: with_seeds (0 , 0 , 0 , 0))) ;
};
}
