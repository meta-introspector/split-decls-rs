macro_rules! deps {
    () => {
        Program!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        # [doc = " Builder"] impl Program { # [doc = " By default `stderr` of programs is inherited and typically displayed in the terminal."] pub fn suppress_stderr (mut self) -> Self { self . stderr = false ; self } }
    };
}

impl_25!()