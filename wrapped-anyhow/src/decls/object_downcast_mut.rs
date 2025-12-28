macro_rules! deps {
    () => {
        ErrorImpl!();
        Mut!();
    };
}

macro_rules! object_downcast_mut {
    () => {
        deps!();
        # [cfg (anyhow_no_ptr_addr_of)] unsafe fn object_downcast_mut < E > (e : Mut < ErrorImpl > , target : TypeId) -> Option < Mut < () > > where E : 'static , { if TypeId :: of :: < E > () == target { let unerased_mut = e . cast :: < ErrorImpl < E > > () ; let unerased = unsafe { unerased_mut . deref_mut () } ; Some (Mut :: new (& mut unerased . _object) . cast :: < () > ()) } else { None } }
    };
}

object_downcast_mut!();