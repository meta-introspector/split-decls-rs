macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! ISO_8859_10 {
    () => {
        deps!();
        # [doc = " The ISO-8859-10 encoding."] # [doc = ""] # [doc = " This is the Nordic part of the ISO/IEC 8859 encoding family. This encoding"] # [doc = " is also known as Latin 6."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/iso-8859-10.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-10-bmp.html)"] # [doc = ""] # [doc = " The Windows code page number for this encoding is 28600, but kernel32.dll"] # [doc = " does not support this encoding."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static ISO_8859_10 : & 'static Encoding = & ISO_8859_10_INIT ;
    };
}

ISO_8859_10!();