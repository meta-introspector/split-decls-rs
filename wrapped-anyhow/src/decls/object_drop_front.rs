macro_rules! deps {
    () => {
        ErrorImpl!();
        Own!();
    };
}

macro_rules! object_drop_front {
    () => {
        deps!();
        unsafe fn object_drop_front < E > (e : Own < ErrorImpl > , target : TypeId) { let _ = target ; let unerased_own = e . cast :: < ErrorImpl < ManuallyDrop < E > > > () ; drop (unsafe { unerased_own . boxed () }) ; }
    };
}

object_drop_front!()