macro_rules! deps {
    () => {
        AutoCfg!();
    };
}

macro_rules! emit_possibility {
    () => {
        deps!();
        # [doc = " Indicates to rustc that a config flag should not generate an [`unexpected_cfgs` warning][check-cfg-flags]"] # [doc = ""] # [doc = " This looks like `cargo:rustc-check-cfg=cfg(VAR)`"] # [doc = ""] # [doc = " As of rust 1.80, the compiler does [automatic checking of cfgs at compile time][check-cfg-flags]."] # [doc = " All custom configuration flags must be known to rustc, or they will generate a warning."] # [doc = " This is done automatically when calling the builtin emit methods on [`AutoCfg`],"] # [doc = " but not when calling [`autocfg::emit`](crate::emit) directly."] # [doc = ""] # [doc = " Versions before rust 1.80 will simply ignore this directive."] # [doc = ""] # [doc = " This function indicates to the compiler that the config flag never has a value."] # [doc = " If this is not desired, see [the blog post][check-cfg]."] # [doc = ""] # [doc = " [check-cfg-flags]: https://blog.rust-lang.org/2024/05/06/check-cfg.html"] pub fn emit_possibility (cfg : & str) { println ! ("cargo:rustc-check-cfg=cfg({})" , cfg) ; }
    };
}

emit_possibility!();