macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! WINDOWS_874 {
    () => {
        deps!();
        # [doc = " The windows-874 encoding."] # [doc = ""] # [doc = " This is the Thai encoding for Windows. It is an extension of TIS-620 / ISO-8859-11."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/windows-874.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-874-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 874, except Windows decodes"] # [doc = " unassigned code points to the Private Use Area of Unicode."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static WINDOWS_874 : & 'static Encoding = & WINDOWS_874_INIT ;
    };
}

WINDOWS_874!()