macro_rules! PrintFmt {
    () => {
        # [doc = " The styles of printing that we can print"] # [derive (Copy , Clone , Eq , PartialEq)] # [non_exhaustive] pub enum PrintFmt { # [doc = " Prints a terser backtrace which ideally only contains relevant information"] Short , # [doc = " Prints a backtrace that contains all possible information"] Full , }
    };
}

PrintFmt!();