macro_rules! Buf {
    () => {
        # [doc = " A structure to wrap an intermediate buffer used by libgit2."] # [doc = ""] # [doc = " A buffer can be thought of a `Vec<u8>`, but the `Vec` type is not used to"] # [doc = " avoid copying data back and forth."] pub struct Buf { raw : raw :: git_buf , }
    };
}

Buf!()