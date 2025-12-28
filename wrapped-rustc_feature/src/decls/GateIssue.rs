macro_rules! GateIssue {
    () => {
        pub enum GateIssue { Language , Library (Option < NonZero < u32 > >) , }
    };
}

GateIssue!()