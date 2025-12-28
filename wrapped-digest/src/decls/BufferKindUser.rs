macro_rules! BufferKindUser {
    () => {
        # [doc = " Types which use [`BlockBuffer`] functionality."] pub trait BufferKindUser : BlockSizeUser { # [doc = " Block buffer kind over which type operates."] type BufferKind : BufferKind ; }
    };
}

BufferKindUser!();