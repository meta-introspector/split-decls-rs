macro_rules! deps {
    () => {
        ErrorVTable!();
        ErrorImpl!();
    };
}

macro_rules! vtable {
    () => {
        deps!();
        unsafe fn vtable (p : NonNull < ErrorImpl >) -> & 'static ErrorVTable { unsafe { * (p . as_ptr () as * const & 'static ErrorVTable) } }
    };
}

vtable!()