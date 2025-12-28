macro_rules! deps {
    () => {
        ErrorImpl!();
        Own!();
    };
}

macro_rules! object_drop {
    () => {
        deps!();
        unsafe fn object_drop < E > (e : Own < ErrorImpl >) { let unerased_own = e . cast :: < ErrorImpl < E > > () ; drop (unsafe { unerased_own . boxed () }) ; }
    };
}

object_drop!();