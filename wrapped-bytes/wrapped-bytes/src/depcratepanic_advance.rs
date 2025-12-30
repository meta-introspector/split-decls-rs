// Generated macro for panic_advance (function)
macro_rules! Depcratepanic_advance {
() => {
// Module: crate
// Provides: {"panic_advance"}
// Dependencies: {}
# [doc = " Panic with a nice error message."] # [cold] fn panic_advance (error_info : & TryGetError) -> ! { panic ! ("advance out of bounds: the len is {} but advancing by {}" , error_info . available , error_info . requested) ; }
};
}
