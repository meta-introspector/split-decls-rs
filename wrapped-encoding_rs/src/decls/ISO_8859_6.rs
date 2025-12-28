macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! ISO_8859_6 {
    () => {
        deps!();
        # [doc = " The ISO-8859-6 encoding."] # [doc = ""] # [doc = " This is the Arabic part of the ISO/IEC 8859 encoding family."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/iso-8859-6.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-6-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 28596, except Windows decodes"] # [doc = " unassigned code points to the Private Use Area of Unicode."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static ISO_8859_6 : & 'static Encoding = & ISO_8859_6_INIT ;
    };
}

ISO_8859_6!()