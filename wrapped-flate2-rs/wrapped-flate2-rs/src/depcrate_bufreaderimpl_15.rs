// Generated macro for impl_15 (impl)
macro_rules! Depcrate_bufreaderimpl_15 {
() => {
// Module: crate::bufreader
// Provides: {"impl_15"}
// Dependencies: {}
impl < R > BufReader < R > { pub fn get_ref (& self) -> & R { & self . inner } pub fn get_mut (& mut self) -> & mut R { & mut self . inner } pub fn into_inner (self) -> R { self . inner } pub fn reset (& mut self , inner : R) -> R { self . pos = 0 ; self . cap = 0 ; mem :: replace (& mut self . inner , inner) } }
};
}
