macro_rules! deps {
    () => {
        RealExecv!();
        Result!();
        Execv!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Execv for RealExecv { fn execv (& self , program : & OsStr , args : & [& OsStr] , current_dir : Option < & Path > ,) -> Result < Output > { let mut command = Command :: new (program) ; command . args (args) ; if let Some (dir) = current_dir { command . current_dir (dir) ; } Ok (command . output () ?) } }
    };
}

impl_8!()