macro_rules! deps {
    () => {
        StdError!();
        ErrorImpl!();
        Own!();
    };
}

macro_rules! object_boxed {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] unsafe fn object_boxed < E > (e : Own < ErrorImpl >) -> Box < dyn StdError + Send + Sync + 'static > where E : StdError + Send + Sync + 'static , { let unerased_own = e . cast :: < ErrorImpl < E > > () ; unsafe { unerased_own . boxed () } }
    };
}

object_boxed!()