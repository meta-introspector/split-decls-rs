macro_rules! deps {
    () => {
        Process!();
    };
}

macro_rules! ProcessSrvState {
    () => {
        deps!();
        # [doc = " Maintains the state of the proc-macro server process."] # [derive (Debug)] struct ProcessSrvState { process : Process , stdin : ChildStdin , stdout : BufReader < ChildStdout > , }
    };
}

ProcessSrvState!();