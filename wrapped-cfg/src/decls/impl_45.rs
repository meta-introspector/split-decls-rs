macro_rules! deps {
    () => {
        CfgOptions!();
        CfgAtom!();
        HashableCfgOptions!();
        CfgDiff!();
        CfgExpr!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl CfgOptions { pub fn check (& self , cfg : & CfgExpr) -> Option < bool > { cfg . fold (& | atom | self . enabled . contains (atom)) } pub fn check_atom (& self , cfg : & CfgAtom) -> bool { self . enabled . contains (cfg) } pub fn insert_atom (& mut self , key : Symbol) { self . insert_any_atom (CfgAtom :: Flag (key)) ; } pub fn insert_key_value (& mut self , key : Symbol , value : Symbol) { self . insert_any_atom (CfgAtom :: KeyValue { key , value }) ; } pub fn apply_diff (& mut self , diff : CfgDiff) { for atom in diff . enable { self . insert_any_atom (atom) ; } for atom in diff . disable { let (CfgAtom :: Flag (sym) | CfgAtom :: KeyValue { key : sym , .. }) = & atom ; if * sym == sym :: true_ || * sym == sym :: false_ { tracing :: error ! ("cannot remove `true` or `false` from cfg") ; continue ; } self . enabled . remove (& atom) ; } } fn insert_any_atom (& mut self , atom : CfgAtom) { let (CfgAtom :: Flag (sym) | CfgAtom :: KeyValue { key : sym , .. }) = & atom ; if * sym == sym :: true_ || * sym == sym :: false_ { tracing :: error ! ("cannot insert `true` or `false` to cfg") ; return ; } self . enabled . insert (atom) ; } pub fn get_cfg_keys (& self) -> impl Iterator < Item = & Symbol > { self . enabled . iter () . map (| it | match it { CfgAtom :: Flag (key) => key , CfgAtom :: KeyValue { key , .. } => key , }) } pub fn get_cfg_values < 'a > (& 'a self , cfg_key : & 'a str) -> impl Iterator < Item = & 'a Symbol > + 'a { self . enabled . iter () . filter_map (move | it | match it { CfgAtom :: KeyValue { key , value } if cfg_key == key . as_str () => Some (value) , _ => None , }) } pub fn to_hashable (& self) -> HashableCfgOptions { let mut enabled = self . enabled . iter () . cloned () . collect :: < Box < [_] > > () ; enabled . sort_unstable () ; HashableCfgOptions { _enabled : enabled } } # [inline] pub fn shrink_to_fit (& mut self) { self . enabled . shrink_to_fit () ; } pub fn append (& mut self , other : CfgOptions) { self . enabled . extend (other . enabled) ; } }
    };
}

impl_45!();