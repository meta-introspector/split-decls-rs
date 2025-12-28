macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! Program {
    () => {
        deps!();
        # [doc = " A program/executable implementing the credential helper protocol."] # [derive (Debug)] pub struct Program { # [doc = " The kind of program, ready for launch."] pub kind : program :: Kind , # [doc = " If true, stderr is enabled, which is the default."] pub stderr : bool , # [doc = " `Some(…)` if the process is running."] child : Option < std :: process :: Child > , }
    };
}

Program!()