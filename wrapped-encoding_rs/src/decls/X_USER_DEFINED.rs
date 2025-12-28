macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! X_USER_DEFINED {
    () => {
        deps!();
        # [doc = " The x-user-defined encoding."] # [doc = ""] # [doc = " This encoding offsets the non-ASCII bytes by `0xF700` thereby decoding"] # [doc = " them to the Private Use Area of Unicode. It was used for loading binary"] # [doc = " data into a JavaScript string using `XMLHttpRequest` before XHR supported"] # [doc = " the `\"arraybuffer\"` response type."] # [doc = ""] # [doc = " This encoding does not have a Windows code page number."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static X_USER_DEFINED : & 'static Encoding = & X_USER_DEFINED_INIT ;
    };
}

X_USER_DEFINED!()