macro_rules! discover_sysroot_dir {
    () => {
        fn discover_sysroot_dir (current_dir : & AbsPath , extra_env : & FxHashMap < String , Option < String > > ,) -> Result < AbsPathBuf > { let mut rustc = toolchain :: command (Tool :: Rustc . path () , current_dir , extra_env) ; rustc . current_dir (current_dir) . args (["--print" , "sysroot"]) ; tracing :: debug ! ("Discovering sysroot by {:?}" , rustc) ; let stdout = utf8_stdout (& mut rustc) ? ; Ok (AbsPathBuf :: assert (Utf8PathBuf :: from (stdout))) }
    };
}

discover_sysroot_dir!();