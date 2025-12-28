macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! ISO_8859_4 {
    () => {
        deps!();
        # [doc = " The ISO-8859-4 encoding."] # [doc = ""] # [doc = " This is the North European part of the ISO/IEC 8859 encoding family. This encoding is also known as Latin 4."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/iso-8859-4.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/iso-8859-4-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 28594."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static ISO_8859_4 : & 'static Encoding = & ISO_8859_4_INIT ;
    };
}

ISO_8859_4!();