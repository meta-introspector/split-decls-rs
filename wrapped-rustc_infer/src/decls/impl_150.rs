macro_rules! deps {
    () => {
        TermVid!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl From < ty :: TyVid > for TermVid { fn from (value : ty :: TyVid) -> Self { TermVid :: Ty (value) } }
    };
}

impl_150!();