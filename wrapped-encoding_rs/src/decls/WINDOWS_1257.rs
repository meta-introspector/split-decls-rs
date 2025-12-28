macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! WINDOWS_1257 {
    () => {
        deps!();
        # [doc = " The windows-1257 encoding."] # [doc = ""] # [doc = " This is the Baltic encoding for Windows."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/windows-1257.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1257-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 1257, except Windows decodes"] # [doc = " unassigned code points to the Private Use Area of Unicode."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static WINDOWS_1257 : & 'static Encoding = & WINDOWS_1257_INIT ;
    };
}

WINDOWS_1257!()