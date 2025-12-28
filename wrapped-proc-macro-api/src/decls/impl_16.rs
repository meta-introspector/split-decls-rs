macro_rules! deps {
    () => {
        Process!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Process { # [doc = " Runs a new proc-macro server process with the specified environment variables."] fn run < 'a > (path : & AbsPath , env : impl IntoIterator < Item = (impl AsRef < std :: ffi :: OsStr > , & 'a Option < impl 'a + AsRef < std :: ffi :: OsStr > >) , > ,) -> io :: Result < Process > { let child = JodChild (mk_child (path , env) ?) ; Ok (Process { child }) } # [doc = " Retrieves stdin and stdout handles for the process."] fn stdio (& mut self) -> Option < (ChildStdin , BufReader < ChildStdout >) > { let stdin = self . child . stdin . take () ? ; let stdout = self . child . stdout . take () ? ; let read = BufReader :: new (stdout) ; Some ((stdin , read)) } }
    };
}

impl_16!()