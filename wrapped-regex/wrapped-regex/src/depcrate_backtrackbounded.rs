// Generated macro for Bounded (struct)
macro_rules! Depcrate_backtrackBounded {
() => {
// Module: crate::backtrack
// Provides: {"Bounded"}
// Dependencies: {}
# [doc = " A backtracking matching engine."] # [derive (Debug)] pub struct Bounded < 'a , 'm , 'r , 's , I > { prog : & 'r Program , input : I , matches : & 'm mut [bool] , slots : & 's mut [Slot] , m : & 'a mut Cache , }
};
}
