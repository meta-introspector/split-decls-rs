macro_rules! deps {
    () => {
        Language!();
        PreorderWithTokens!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < L : Language > PreorderWithTokens < L > { pub fn skip_subtree (& mut self) { self . raw . skip_subtree () } }
    };
}

impl_75!()