// Generated macro for no_delay_panic (function)
macro_rules! Depcrate_spino_delay_panic {
() => {
// Module: crate::spi
// Provides: {"no_delay_panic"}
// Dependencies: {}
# [cold] fn no_delay_panic () { panic ! ("You've tried to execute a SPI transaction containing a `Operation::DelayNs` in a `SpiDevice` created with `new_no_delay()`. Create it with `new()` instead, passing a `DelayNs` implementation.") ; }
};
}
