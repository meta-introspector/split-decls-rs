macro_rules! CheckAlignment {
    () => {
        # [derive (Copy , Clone)] pub enum CheckAlignment { # [doc = " Ignore all alignment requirements."] # [doc = " This is mainly used in interning."] No , # [doc = " Hard error when dereferencing a misaligned pointer."] Error , }
    };
}

CheckAlignment!();