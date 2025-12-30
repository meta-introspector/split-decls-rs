// Generated macro for impl_283 (impl)
macro_rules! Depcrate_stringimpl_283 {
() => {
// Module: crate::string
// Provides: {"impl_283"}
// Dependencies: {}
impl < LenT : LenType , const N : usize > str :: FromStr for String < N , LenT > { type Err = CapacityError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut new = Self :: new () ; new . push_str (s) ? ; Ok (new) } }
};
}
