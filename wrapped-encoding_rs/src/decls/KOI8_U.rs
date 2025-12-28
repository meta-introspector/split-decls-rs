macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! KOI8_U {
    () => {
        deps!();
        # [doc = " The KOI8-U encoding."] # [doc = ""] # [doc = " This is an encoding for Ukrainian adapted from KOI8-R."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/koi8-u.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/koi8-u-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 21866."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static KOI8_U : & 'static Encoding = & KOI8_U_INIT ;
    };
}

KOI8_U!()