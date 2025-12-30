// Generated macro for alloc_fully_below_2g (function)
macro_rules! Depcratealloc_fully_below_2g {
() => {
// Module: crate
// Provides: {"alloc_fully_below_2g"}
// Dependencies: {}
fn alloc_fully_below_2g (ptr : * mut u8 , layout : Layout) -> bool { ! pointer_above_2g (ptr) && ! pointer_above_2g (unsafe { ptr . add (layout . size () - 1) }) }
};
}
