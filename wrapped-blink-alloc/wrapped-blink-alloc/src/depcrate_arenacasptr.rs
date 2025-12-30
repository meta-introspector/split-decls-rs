// Generated macro for CasPtr (trait)
macro_rules! Depcrate_arenaCasPtr {
() => {
// Module: crate::arena
// Provides: {"CasPtr"}
// Dependencies: {}
pub (crate) trait CasPtr { # [allow (dead_code)] fn new (value : * mut u8) -> Self ; fn load (& self , order : Ordering) -> * mut u8 ; fn set (& mut self , value : * mut u8) ; fn compare_exchange (& self , old : * mut u8 , new : * mut u8 , success : Ordering , failure : Ordering ,) -> Result < () , * mut u8 > ; fn compare_exchange_weak (& self , old : * mut u8 , new : * mut u8 , success : Ordering , failure : Ordering ,) -> Result < () , * mut u8 > ; }
};
}
