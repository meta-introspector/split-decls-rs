macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! SHIFT_JIS {
    () => {
        deps!();
        # [doc = " The Shift_JIS encoding."] # [doc = ""] # [doc = " This is the Japanese encoding for Windows."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/shift_jis.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/shift_jis-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 932, except Windows decodes some byte"] # [doc = " sequences that are error per the Encoding Standard to the question mark or the"] # [doc = " Private Use Area and generally uses U+30FB in place of the REPLACEMENT CHARACTER."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static SHIFT_JIS : & 'static Encoding = & SHIFT_JIS_INIT ;
    };
}

SHIFT_JIS!()