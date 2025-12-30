// Generated macro for in_place (function)
macro_rules! Depcratein_place {
() => {
// Module: crate
// Provides: {"in_place"}
// Dependencies: {}
# [inline] unsafe fn in_place < 'a , T , I > (ptr : * mut T , init : I , f : impl FnOnce (I) -> T) -> & 'a mut T { core :: ptr :: write (ptr , f (init)) ; & mut * ptr }
};
}
