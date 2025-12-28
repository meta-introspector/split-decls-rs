macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! WINDOWS_1256 {
    () => {
        deps!();
        # [doc = " The windows-1256 encoding."] # [doc = ""] # [doc = " This is the Arabic encoding for Windows."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/windows-1256.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/windows-1256-bmp.html)"] # [doc = ""] # [doc = " This encoding matches the Windows code page 1256."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static WINDOWS_1256 : & 'static Encoding = & WINDOWS_1256_INIT ;
    };
}

WINDOWS_1256!()