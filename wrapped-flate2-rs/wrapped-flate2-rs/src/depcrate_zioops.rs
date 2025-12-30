// Generated macro for Ops (trait)
macro_rules! Depcrate_zioOps {
() => {
// Module: crate::zio
// Provides: {"Ops"}
// Dependencies: {}
pub trait Ops { type Error : Into < io :: Error > ; type Flush : Flush ; fn total_in (& self) -> u64 ; fn total_out (& self) -> u64 ; fn run (& mut self , input : & [u8] , output : & mut [u8] , flush : Self :: Flush ,) -> Result < Status , Self :: Error > ; fn run_vec (& mut self , input : & [u8] , output : & mut Vec < u8 > , flush : Self :: Flush ,) -> Result < Status , Self :: Error > ; }
};
}
