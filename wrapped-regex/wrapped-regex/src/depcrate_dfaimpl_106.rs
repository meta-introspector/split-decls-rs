// Generated macro for impl_106 (impl)
macro_rules! Depcrate_dfaimpl_106 {
() => {
// Module: crate::dfa
// Provides: {"impl_106"}
// Dependencies: {}
impl fmt :: Debug for State { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let ips : Vec < usize > = self . inst_ptrs () . collect () ; f . debug_struct ("State") . field ("flags" , & self . flags ()) . field ("insts" , & ips) . finish () } }
};
}
