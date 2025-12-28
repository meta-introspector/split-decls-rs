macro_rules! deps {
    () => {
        Unicode!();
        Encoding!();
    };
}

macro_rules! ISO_8859_7 {
    () => {
        deps!();
        # [doc = " The ISO-8859-7 encoding."] # [doc = ""] # [doc = " This is the Greek part of the ISO/IEC 8859 encoding family."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/iso-8859-7.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-7-bmp.html)"] # [doc = ""] # [doc = " This encoding roughly matches the Windows code page 28597. Windows decodes"] # [doc = " unassigned code points, the currency signs at 0xA4 and 0xA5 as well as"] # [doc = " 0xAA, which should be U+037A GREEK YPOGEGRAMMENI, to the Private Use Area"] # [doc = " of Unicode. Windows decodes 0xA1 to U+02BD MODIFIER LETTER REVERSED COMMA"] # [doc = " instead of U+2018 LEFT SINGLE QUOTATION MARK and 0xA2 to U+02BC MODIFIER"] # [doc = " LETTER APOSTROPHE instead of U+2019 RIGHT SINGLE QUOTATION MARK."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static ISO_8859_7 : & 'static Encoding = & ISO_8859_7_INIT ;
    };
}

ISO_8859_7!();