macro_rules! rustc_path {
    () => {
        pub fn rustc_path < 'a > (sysroot : & Sysroot) -> Option < & 'a Path > { static RUSTC_PATH : OnceLock < Option < PathBuf > > = OnceLock :: new () ; RUSTC_PATH . get_or_init (| | { let candidate = sysroot . default . join (env ! ("RUSTC_INSTALL_BINDIR")) . join (if cfg ! (target_os = "windows") { "rustc.exe" } else { "rustc" }) ; candidate . exists () . then_some (candidate) }) . as_deref () }
    };
}

rustc_path!();