macro_rules! deps {
    () => {
        Ref!();
        ErrorImpl!();
    };
}

macro_rules! object_downcast {
    () => {
        deps!();
        unsafe fn object_downcast < E > (e : Ref < ErrorImpl > , target : TypeId) -> Option < Ref < () > > where E : 'static , { if TypeId :: of :: < E > () == target { let unerased_ref = e . cast :: < ErrorImpl < E > > () ; # [cfg (not (anyhow_no_ptr_addr_of))] return Some (Ref :: from_raw (unsafe { NonNull :: new_unchecked (ptr :: addr_of ! ((* unerased_ref . as_ptr ()) . _object) as * mut E) }) . cast :: < () > () ,) ; # [cfg (anyhow_no_ptr_addr_of)] return Some (Ref :: new (unsafe { & unerased_ref . deref () . _object }) . cast :: < () > ()) ; } else { None } }
    };
}

object_downcast!();