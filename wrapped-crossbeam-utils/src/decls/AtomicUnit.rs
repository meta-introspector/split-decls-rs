macro_rules! AtomicUnit {
    () => {
        # [doc = " An atomic `()`."] # [doc = ""] # [doc = " All operations are noops."] struct AtomicUnit ;
    };
}

AtomicUnit!();