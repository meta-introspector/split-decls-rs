macro_rules! deps {
    () => {
        Demangle!();
    };
}

macro_rules! DemangleStyle {
    () => {
        deps!();
        enum DemangleStyle < 'a > { Legacy (legacy :: Demangle < 'a >) , V0 (v0 :: Demangle < 'a >) , }
    };
}

DemangleStyle!();