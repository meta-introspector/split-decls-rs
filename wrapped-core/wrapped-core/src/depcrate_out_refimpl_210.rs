// Generated macro for impl_210 (impl)
macro_rules! Depcrate_out_refimpl_210 {
() => {
// Module: crate::out_ref
// Provides: {"impl_210"}
// Dependencies: {}
impl < T : Type < T > > OutRef < '_ , T > { # [doc = " Returns `true` if the argument is null."] pub fn is_null (& self) -> bool { self . 0 . is_null () } # [doc = " Overwrites a memory location with the given value without reading or dropping the old value."] pub fn write (self , value : T :: Default) -> Result < () > { if self . 0 . is_null () { Err (Error :: from_hresult (imp :: E_POINTER)) } else { unsafe { * self . 0 = core :: mem :: transmute_copy (& value) } core :: mem :: forget (value) ; Ok (()) } } }
};
}
