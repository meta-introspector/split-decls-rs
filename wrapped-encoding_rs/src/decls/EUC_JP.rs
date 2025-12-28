macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! EUC_JP {
    () => {
        deps!();
        # [doc = " The EUC-JP encoding."] # [doc = ""] # [doc = " This is the legacy Unix encoding for Japanese."] # [doc = ""] # [doc = " For compatibility with Web servers that don't expect three-byte sequences"] # [doc = " in form submissions, the encoder doesn't generate three-byte sequences."] # [doc = " That is, the JIS X 0212 support is decode-only."] # [doc = ""] # [doc = " [Index visualization](https://encoding.spec.whatwg.org/euc-jp.html),"] # [doc = " [Visualization of BMP coverage](https://encoding.spec.whatwg.org/euc-jp-bmp.html)"] # [doc = ""] # [doc = " This encoding roughly matches the Windows code page 20932. There are error"] # [doc = " handling differences and a handful of 2-byte sequences that decode differently."] # [doc = " Additionall, Windows doesn't support 3-byte sequences."] # [doc = ""] # [doc = " This will change from `static` to `const` if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate, so don't take the address of this"] # [doc = " `static`."] pub static EUC_JP : & 'static Encoding = & EUC_JP_INIT ;
    };
}

EUC_JP!()