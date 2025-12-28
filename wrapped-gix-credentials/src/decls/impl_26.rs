macro_rules! deps {
    () => {
        Error!();
        Program!();
        Action!();
        Result!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Program { pub (crate) fn start (& mut self , action : & helper :: Action ,) -> std :: io :: Result < (std :: process :: ChildStdin , Option < std :: process :: ChildStdout >) > { assert ! (self . child . is_none () , "BUG: must not call `start()` twice") ; let mut cmd = self . to_command (action) ; gix_trace :: debug ! (cmd = ? cmd , "launching credential helper") ; let mut child = cmd . spawn () ? ; let stdin = child . stdin . take () . expect ("stdin to be configured") ; let stdout = child . stdout . take () ; self . child = child . into () ; Ok ((stdin , stdout)) } pub (crate) fn finish (& mut self) -> std :: io :: Result < () > { let mut child = self . child . take () . expect ("Call `start()` before calling finish()") ; let status = child . wait () ? ; if status . success () { Ok (()) } else { Err (std :: io :: Error :: other (format ! ("Credentials helper program failed with status code {:?}" , status . code ()))) } } }
    };
}

impl_26!();