// Generated macro for enable (function)
macro_rules! Depcrateenable {
() => {
// Module: crate
// Provides: {"enable"}
// Dependencies: {}
# [doc = " Enables defmt logging over the ITM stimulus port 0."] # [doc = ""] # [doc = " This needs to be called by the application before defmt logging is used, otherwise the logs will be disposed."] pub fn enable (itm : ITM) { unsafe { itm . ter [0] . write (1) } ENABLED . store (true , Ordering :: Relaxed) ; }
};
}
