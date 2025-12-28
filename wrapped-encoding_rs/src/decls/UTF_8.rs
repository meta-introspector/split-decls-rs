macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! UTF_8 {
    () => {
        deps!();
        # [doc = " The UTF-8 encoding."] # [doc = ""] # [doc = " This is the encoding that should be used for all new development it can"] # [doc = " represent all of Unicode."] # [doc = ""] # [doc = " This encoding matches the Windows code page 65001, except Windows differs"] # [doc = " in the number of errors generated for some erroneous byte sequences."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static UTF_8 : & 'static Encoding = & UTF_8_INIT ;
    };
}

UTF_8!()