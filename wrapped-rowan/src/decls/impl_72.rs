macro_rules! deps {
    () => {
        Language!();
        Preorder!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < L : Language > Preorder < L > { pub fn skip_subtree (& mut self) { self . raw . skip_subtree () } }
    };
}

impl_72!();