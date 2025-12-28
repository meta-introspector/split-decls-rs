macro_rules! deps {
    () => {
        MirPhase!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl IntoDiagArg for MirPhase { fn into_diag_arg (self , _path : & mut Option < PathBuf >) -> DiagArgValue { let arg = match self { MirPhase :: Initial => "initial" , MirPhase :: PostCleanup => "post-cleanup" , MirPhase :: Optimized => "optimized" , } ; DiagArgValue :: Str (Cow :: Borrowed (arg)) } }
    };
}

impl_25!()