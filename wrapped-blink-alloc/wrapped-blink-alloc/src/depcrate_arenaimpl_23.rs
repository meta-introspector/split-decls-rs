// Generated macro for impl_23 (impl)
macro_rules! Depcrate_arenaimpl_23 {
() => {
// Module: crate::arena
// Provides: {"impl_23"}
// Dependencies: {}
impl CasPtr for AtomicPtr < u8 > { # [inline (always)] fn new (value : * mut u8) -> Self { AtomicPtr :: new (value) } # [inline (always)] fn load (& self , order : Ordering) -> * mut u8 { self . load (order) } # [inline (always)] fn set (& mut self , value : * mut u8) { * self . get_mut () = value ; } # [inline (always)] fn compare_exchange (& self , old : * mut u8 , new : * mut u8 , success : Ordering , failure : Ordering ,) -> Result < () , * mut u8 > { self . compare_exchange (old , new , success , failure) ? ; Ok (()) } # [inline (always)] fn compare_exchange_weak (& self , old : * mut u8 , new : * mut u8 , success : Ordering , failure : Ordering ,) -> Result < () , * mut u8 > { self . compare_exchange_weak (old , new , success , failure) ? ; Ok (()) } }
};
}
