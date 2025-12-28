macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! ISO_8859_16 {
    () => {
        deps!();
        # [doc = " The ISO-8859-16 encoding."] # [doc = ""] # [doc = " This is the South-Eastern European part of the ISO/IEC 8859 encoding"] # [doc = " family. This encoding is also known as Latin 10."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/iso-8859-16.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-16-bmp.html)"] # [doc = ""] # [doc = " The Windows code page number for this encoding is 28606, but kernel32.dll"] # [doc = " does not support this encoding."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static ISO_8859_16 : & 'static Encoding = & ISO_8859_16_INIT ;
    };
}

ISO_8859_16!()