// Generated macro for Wait2 (struct)
macro_rules! Depcrate_waiter_waitWait2 {
() => {
// Module: crate::waiter::wait
// Provides: {"Wait2"}
// Dependencies: {}
# [derive (Debug)] pub struct Wait2 < R1 , R2 > { recv : Receiver < (usize , io :: Result < Option < u8 > >) > , b1 : Blocking < R1 > , b2 : Blocking < R2 > , timeout : Duration , }
};
}
