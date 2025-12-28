macro_rules! deps {
    () => {
        ProcMacroLoadingError!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Error for ProcMacroLoadingError { }
    };
}

impl_9!()