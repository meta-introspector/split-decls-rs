macro_rules! deps {
    () => {
        StrippedCfgItem!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < ModId > StrippedCfgItem < ModId > { pub fn map_mod_id < New > (self , f : impl FnOnce (ModId) -> New) -> StrippedCfgItem < New > { StrippedCfgItem { parent_module : f (self . parent_module) , ident : self . ident , cfg : self . cfg } } }
    };
}

impl_19!()