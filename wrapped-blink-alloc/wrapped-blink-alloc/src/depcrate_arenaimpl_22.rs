// Generated macro for impl_22 (impl)
macro_rules! Depcrate_arenaimpl_22 {
() => {
// Module: crate::arena
// Provides: {"impl_22"}
// Dependencies: {}
impl CasPtr for Cell < * mut u8 > { # [inline (always)] fn new (value : * mut u8) -> Self { Cell :: new (value) } # [inline (always)] fn load (& self , _ : Ordering) -> * mut u8 { self . get () } # [inline (always)] fn set (& mut self , value : * mut u8) { * self . get_mut () = value ; } # [inline (always)] fn compare_exchange (& self , old : * mut u8 , new : * mut u8 , _ : Ordering , _ : Ordering ,) -> Result < () , * mut u8 > { if old == self . get () { self . set (new) ; Ok (()) } else { Err (self . get ()) } } # [inline (always)] fn compare_exchange_weak (& self , old : * mut u8 , new : * mut u8 , _ : Ordering , _ : Ordering ,) -> Result < () , * mut u8 > { debug_assert_eq ! (old , self . get () , "Must be used only in loop where `old` is last loaded value") ; self . set (new) ; Ok (()) } }
};
}
