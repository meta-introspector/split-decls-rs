macro_rules! deps {
    () => {
        TermVid!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl From < ty :: ConstVid > for TermVid { fn from (value : ty :: ConstVid) -> Self { TermVid :: Const (value) } }
    };
}

impl_151!()