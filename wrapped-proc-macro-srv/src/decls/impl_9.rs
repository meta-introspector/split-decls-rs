macro_rules! deps {
    () => {
        ProcMacroSrv!();
        EnvSnapshot!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'env > ProcMacroSrv < 'env > { pub fn new (env : & 'env EnvSnapshot) -> Self { Self { expanders : Default :: default () , env , temp_dir : TempDir :: with_prefix ("proc-macro-srv") . unwrap () , } } }
    };
}

impl_9!()