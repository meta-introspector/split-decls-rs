macro_rules! deps {
    () => {
        Own!();
        StdError!();
        ErrorImpl!();
    };
}

macro_rules! object_reallocate_boxed {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] unsafe fn object_reallocate_boxed < E > (e : Own < ErrorImpl >) -> Box < dyn StdError + Send + Sync + 'static > where E : StdError + Send + Sync + 'static , { let unerased_own = e . cast :: < ErrorImpl < E > > () ; Box :: new (unsafe { unerased_own . boxed () } . _object) }
    };
}

object_reallocate_boxed!()