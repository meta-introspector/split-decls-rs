// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl < B : BitBlock > fmt :: Debug for BitVec < B > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { self . ensure_invariant () ; let mut storage = String :: with_capacity (self . len () + self . len () / B :: bits ()) ; for (i , bit) in self . iter () . enumerate () { if i != 0 && i % B :: bits () == 0 { storage . push (' ') ; } storage . push (if bit { '1' } else { '0' }) ; } fmt . debug_struct ("BitVec") . field ("storage" , & storage) . field ("nbits" , & self . nbits) . finish () } }
};
}
