macro_rules! discover_rust_lib_src_dir_or_add_component {
    () => {
        fn discover_rust_lib_src_dir_or_add_component (sysroot_path : & AbsPathBuf , current_dir : & AbsPath , extra_env : & FxHashMap < String , Option < String > > ,) -> Result < AbsPathBuf > { discover_rust_lib_src_dir (sysroot_path) . or_else (| | { let mut rustup = toolchain :: command (Tool :: Rustup . prefer_proxy () , current_dir , extra_env) ; rustup . args (["component" , "add" , "rust-src"]) ; tracing :: info ! ("adding rust-src component by {:?}" , rustup) ; utf8_stdout (& mut rustup) . ok () ? ; get_rust_lib_src (sysroot_path) }) . ok_or_else (| | { tracing :: error ! (% sysroot_path , "can't load standard library, try installing `rust-src`") ; format_err ! ("\
can't load standard library from sysroot
{sysroot_path}
(discovered via `rustc --print sysroot`)
try installing `rust-src` the same way you installed `rustc`") }) }
    };
}

discover_rust_lib_src_dir_or_add_component!();