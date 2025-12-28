macro_rules! deps {
    () => {
        Mut!();
        StdError!();
        ErrorImpl!();
    };
}

macro_rules! object_mut {
    () => {
        deps!();
        # [cfg (all (any (feature = "std" , not (anyhow_no_core_error)) , anyhow_no_ptr_addr_of))] unsafe fn object_mut < E > (e : Mut < ErrorImpl >) -> & mut (dyn StdError + Send + Sync + 'static) where E : StdError + Send + Sync + 'static , { let unerased_mut = e . cast :: < ErrorImpl < E > > () ; unsafe { & mut unerased_mut . deref_mut () . _object } }
    };
}

object_mut!();