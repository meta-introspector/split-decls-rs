macro_rules! deps {
    () => {
        CfgOptions!();
        CfgAtom!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Extend < CfgAtom > for CfgOptions { fn extend < T : IntoIterator < Item = CfgAtom > > (& mut self , iter : T) { iter . into_iter () . for_each (| cfg_flag | self . insert_any_atom (cfg_flag)) ; } }
    };
}

impl_46!();