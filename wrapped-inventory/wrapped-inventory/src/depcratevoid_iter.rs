// Generated macro for void_iter (module)
macro_rules! Depcratevoid_iter {
() => {
// Module: crate
// Provides: {"void_iter"}
// Dependencies: {}
mod void_iter { enum Void { } # [repr (C , packed)] pub struct Iter < T > ([* const T ; 0] , Void) ; unsafe impl < T > Send for Iter < T > { } unsafe impl < T > Sync for Iter < T > { } }
};
}
