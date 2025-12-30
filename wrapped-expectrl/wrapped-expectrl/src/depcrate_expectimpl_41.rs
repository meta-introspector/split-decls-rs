// Generated macro for impl_41 (impl)
macro_rules! Depcrate_expectimpl_41 {
() => {
// Module: crate::expect
// Provides: {"impl_41"}
// Dependencies: {}
impl < T > Expect for & mut T where T : Expect , { fn expect < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { T :: expect (self , needle) } fn check < N > (& mut self , needle : N) -> Result < Captures , Error > where N : Needle , { T :: check (self , needle) } fn is_matched < N > (& mut self , needle : N) -> Result < bool , Error > where N : Needle , { T :: is_matched (self , needle) } fn send < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { T :: send (self , buf) } fn send_line < B > (& mut self , buf : B) -> Result < () , Error > where B : AsRef < [u8] > , { T :: send_line (self , buf) } }
};
}
