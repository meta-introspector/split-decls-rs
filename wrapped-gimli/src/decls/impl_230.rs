macro_rules! deps {
    () => {
        Register!();
        ReaderOffset!();
        RegisterRule!();
        RegisterRuleIter!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < 'iter , T : ReaderOffset > Iterator for RegisterRuleIter < 'iter , T > { type Item = & 'iter (Register , RegisterRule < T >) ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () } }
    };
}

impl_230!();