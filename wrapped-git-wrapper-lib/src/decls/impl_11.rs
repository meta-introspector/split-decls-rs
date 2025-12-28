macro_rules! deps {
    () => {
        Execv!();
        DummyExecv!();
        Result!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Execv for DummyExecv { fn execv (& self , _program : & OsStr , _args : & [& OsStr] , _current_dir : Option < & Path > ,) -> Result < Output > { Ok (Output { status : std :: process :: ExitStatus :: from_raw (0) , stdout : "dummy stdout" . as_bytes () . to_vec () , stderr : "dummy stderr" . as_bytes () . to_vec () , }) } }
    };
}

impl_11!();