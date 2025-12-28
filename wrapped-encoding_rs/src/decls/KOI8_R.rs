macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! KOI8_R {
    () => {
        deps!();
        # [doc = " The KOI8-R encoding."] # [doc = ""] # [doc = " This is an encoding for Russian from [RFC 1489](https://tools.ietf.org/html/rfc1489)."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/koi8-r.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/koi8-r-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 20866."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static KOI8_R : & 'static Encoding = & KOI8_R_INIT ;
    };
}

KOI8_R!();