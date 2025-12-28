macro_rules! deps {
    () => {
        CfgAtom!();
        CfgOptions!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl FromIterator < CfgAtom > for CfgOptions { fn from_iter < T : IntoIterator < Item = CfgAtom > > (iter : T) -> Self { let mut options = CfgOptions :: default () ; options . extend (iter) ; options } }
    };
}

impl_49!();