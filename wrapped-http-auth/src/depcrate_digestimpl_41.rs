// Generated macro for impl_41 (impl)
macro_rules! Depcrate_digestimpl_41 {
() => {
// Module: crate::digest
// Provides: {"impl_41"}
// Dependencies: {}
impl std :: fmt :: Debug for QopSet { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut l = f . debug_set () ; if (self . 0 & Qop :: Auth as u8) != 0 { l . entry (& "auth") ; } if (self . 0 & Qop :: AuthInt as u8) != 0 { l . entry (& "auth-int") ; } l . finish () } }
};
}
