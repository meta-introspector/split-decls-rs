macro_rules! deps {
    () => {
        CfgAtom!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [cfg (test)] impl arbitrary :: Arbitrary < '_ > for CfgAtom { fn arbitrary (u : & mut arbitrary :: Unstructured < '_ >) -> arbitrary :: Result < Self > { if u . arbitrary () ? { Ok (CfgAtom :: Flag (Symbol :: intern (< _ > :: arbitrary (u) ?))) } else { Ok (CfgAtom :: KeyValue { key : Symbol :: intern (< _ > :: arbitrary (u) ?) , value : Symbol :: intern (< _ > :: arbitrary (u) ?) , }) } } }
    };
}

impl_8!();