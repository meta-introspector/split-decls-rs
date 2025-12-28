macro_rules! MainThreadState {
    () => {
        # [derive (PartialEq , Clone , Copy , Debug)] enum MainThreadState { # [doc = " Doing nothing."] Idle , # [doc = " Doing codegen, i.e. MIR-to-LLVM-IR conversion."] Codegenning , # [doc = " Idle, but lending the compiler process's Token to an LLVM thread so it can do useful work."] Lending , }
    };
}

MainThreadState!();