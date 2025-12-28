macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! WINDOWS_1252 {
    () => {
        deps!();
        # [doc = " The windows-1252 encoding."] # [doc = ""] # [doc = " This is the Western encoding for Windows. It is an extension of ISO-8859-1,"] # [doc = " which is known as Latin 1."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/windows-1252.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1252-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 1252."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static WINDOWS_1252 : & 'static Encoding = & WINDOWS_1252_INIT ;
    };
}

WINDOWS_1252!()