macro_rules! deps {
    () => {
        ProcMacroLibrary!();
    };
}

macro_rules! Expander {
    () => {
        deps!();
        pub (crate) struct Expander { inner : ProcMacroLibrary , modified_time : SystemTime , }
    };
}

Expander!()