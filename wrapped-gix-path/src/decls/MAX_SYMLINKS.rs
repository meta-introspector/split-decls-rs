macro_rules! MAX_SYMLINKS {
    () => {
        # [doc = " The default amount of symlinks we may follow when resolving a path in [`realpath()`][crate::realpath()]."] pub const MAX_SYMLINKS : u8 = 32 ;
    };
}

MAX_SYMLINKS!();