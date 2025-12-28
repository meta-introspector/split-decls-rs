macro_rules! deps {
    () => {
        EnvSnapshot!();
        Expander!();
    };
}

macro_rules! ProcMacroSrv {
    () => {
        deps!();
        pub struct ProcMacroSrv < 'env > { expanders : Mutex < HashMap < Utf8PathBuf , Arc < dylib :: Expander > > > , env : & 'env EnvSnapshot , temp_dir : TempDir , }
    };
}

ProcMacroSrv!();