macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! WINDOWS_1255 {
    () => {
        deps!();
        # [doc = " The windows-1255 encoding."] # [doc = ""] # [doc = " This is the Hebrew encoding for Windows. It is an extension of ISO-8859-8-I,"] # [doc = " except for a currency sign swap."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/windows-1255.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1255-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 1255, except Windows decodes"] # [doc = " unassigned code points to the Private Use Area of Unicode."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static WINDOWS_1255 : & 'static Encoding = & WINDOWS_1255_INIT ;
    };
}

WINDOWS_1255!()