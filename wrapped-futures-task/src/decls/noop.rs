macro_rules! noop {
    () => {
        unsafe fn noop (_data : * const ()) { }
    };
}

noop!();