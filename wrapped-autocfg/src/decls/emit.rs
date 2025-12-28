macro_rules! deps {
    () => {
        AutoCfg!();
    };
}

macro_rules! emit {
    () => {
        deps!();
        # [doc = " Writes a config flag for rustc on standard out."] # [doc = ""] # [doc = " This looks like: `cargo:rustc-cfg=CFG`"] # [doc = ""] # [doc = " Cargo will use this in arguments to rustc, like `--cfg CFG`."] # [doc = ""] # [doc = " This does not automatically call [`emit_possibility`]"] # [doc = " so the compiler my generate an [`unexpected_cfgs` warning][check-cfg-flags]."] # [doc = " However, all the builtin emit methods on [`AutoCfg`] call [`emit_possibility`] automatically."] # [doc = ""] # [doc = " [check-cfg-flags]: https://blog.rust-lang.org/2024/05/06/check-cfg.html"] pub fn emit (cfg : & str) { println ! ("cargo:rustc-cfg={}" , cfg) ; }
    };
}

emit!();