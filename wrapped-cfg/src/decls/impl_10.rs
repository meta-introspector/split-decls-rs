macro_rules! deps {
    () => {
        CfgOptions!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl FromIterator < CfgAtom > for CfgOptions { fn from_iter < T : IntoIterator < Item = CfgAtom > > (iter : T) -> Self { let mut options = CfgOptions :: default () ; options . extend (iter) ; options } }
    };
}

impl_10!()