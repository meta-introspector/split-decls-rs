macro_rules! TERMINATOR {
    () => {
        # [doc = " The terminator for each archive member header."] pub const TERMINATOR : [u8 ; 2] = * b"`\n" ;
    };
}

TERMINATOR!();