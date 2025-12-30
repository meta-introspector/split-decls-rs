// Generated macro for TakeExt (trait)
macro_rules! Depcrate_deTakeExt {
() => {
// Module: crate::de
// Provides: {"TakeExt"}
// Dependencies: {}
trait TakeExt { fn take_one (& self) -> Result < (u8 , & [u8]) , Error > ; fn take_n (& self , n : usize) -> Result < (& [u8] , & [u8]) , Error > ; }
};
}
