// Generated macro for impl_282 (impl)
macro_rules! Depcrate_stringimpl_282 {
() => {
// Module: crate::string
// Provides: {"impl_282"}
// Dependencies: {}
impl < 'a , LenT : LenType , const N : usize > TryFrom < & 'a str > for String < N , LenT > { type Error = CapacityError ; fn try_from (s : & 'a str) -> Result < Self , Self :: Error > { let mut new = Self :: new () ; new . push_str (s) ? ; Ok (new) } }
};
}
