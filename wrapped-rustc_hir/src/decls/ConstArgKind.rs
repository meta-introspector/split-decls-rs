macro_rules! deps {
    () => {
        ConstArg!();
        Path!();
        QPath!();
        AnonConst!();
    };
}

macro_rules! ConstArgKind {
    () => {
        deps!();
        # [doc = " See [`ConstArg`]."] # [derive (Clone , Copy , Debug , HashStable_Generic)] # [repr (u8 , C)] pub enum ConstArgKind < 'hir , Unambig = () > { # [doc = " **Note:** Currently this is only used for bare const params"] # [doc = " (`N` where `fn foo<const N: usize>(...)`),"] # [doc = " not paths to any const (`N` where `const N: usize = ...`)."] # [doc = ""] # [doc = " However, in the future, we'll be using it for all of those."] Path (QPath < 'hir >) , Anon (& 'hir AnonConst) , # [doc = " This variant is not always used to represent inference consts, sometimes"] # [doc = " [`GenericArg::Infer`] is used instead."] Infer (Span , Unambig) , }
    };
}

ConstArgKind!()