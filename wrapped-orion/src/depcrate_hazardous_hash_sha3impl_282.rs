// Generated macro for impl_282 (impl)
macro_rules! Depcrate_hazardous_hash_sha3impl_282 {
() => {
// Module: crate::hazardous::hash::sha3
// Provides: {"impl_282"}
// Dependencies: {}
impl < const RATE : usize > Debug for Sha3 < RATE > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "State {{ state: [***OMITTED***], buffer: [***OMITTED***], capacity: {:?}, leftover: {:?}, \
            is_finalized: {:?} }}" , self . capacity , self . leftover , self . is_finalized) } }
};
}
