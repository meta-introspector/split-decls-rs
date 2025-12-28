macro_rules! deps {
    () => {
        ProcMacroLoadResult!();
        ProcMacrosBuilder!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl FromIterator < (CrateBuilderId , ProcMacroLoadResult) > for ProcMacrosBuilder { fn from_iter < T : IntoIterator < Item = (CrateBuilderId , ProcMacroLoadResult) > > (iter : T) -> Self { let mut builder = ProcMacrosBuilder :: default () ; for (k , v) in iter { builder . insert (k , v) ; } builder } }
    };
}

impl_167!()