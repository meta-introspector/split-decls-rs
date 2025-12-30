// Generated macro for leak_and_inhibit_drop (function)
macro_rules! Depcrate_abi_exampleleak_and_inhibit_drop {
() => {
// Module: crate::abi_example
// Provides: {"leak_and_inhibit_drop"}
// Dependencies: {}
fn leak_and_inhibit_drop < 'a , T > (t : T) -> & 'a mut T { Box :: leak (Box :: new (t)) }
};
}
