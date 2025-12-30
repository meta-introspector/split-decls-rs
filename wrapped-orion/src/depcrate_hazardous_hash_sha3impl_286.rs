// Generated macro for impl_286 (impl)
macro_rules! Depcrate_hazardous_hash_sha3impl_286 {
() => {
// Module: crate::hazardous::hash::sha3
// Provides: {"impl_286"}
// Dependencies: {}
impl < const RATE : usize > Debug for Shake < RATE > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "State {{ state: [***OMITTED***], buffer: [***OMITTED***], capacity: {:?}, until_absorb: {:?}, \
            to_squeeze: {:?}, is_finalized: {:?} }}" , self . capacity , self . until_absorb , self . to_squeeze , self . is_finalized) } }
};
}
