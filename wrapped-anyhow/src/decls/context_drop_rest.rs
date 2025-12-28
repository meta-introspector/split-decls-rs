macro_rules! deps {
    () => {
        ContextError!();
        ErrorImpl!();
        Own!();
    };
}

macro_rules! context_drop_rest {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] unsafe fn context_drop_rest < C , E > (e : Own < ErrorImpl > , target : TypeId) where C : 'static , E : 'static , { if TypeId :: of :: < C > () == target { let unerased_own = e . cast :: < ErrorImpl < ContextError < ManuallyDrop < C > , E > > > () ; drop (unsafe { unerased_own . boxed () }) ; } else { let unerased_own = e . cast :: < ErrorImpl < ContextError < C , ManuallyDrop < E > > > > () ; drop (unsafe { unerased_own . boxed () }) ; } }
    };
}

context_drop_rest!()