macro_rules! deps {
    () => {
        Checker!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < 'tcx > Checker < 'tcx > { fn check (& self , trait_def_id : Option < DefId > , f : impl FnOnce (& Self) -> Result < () , ErrorGuaranteed > ,) -> Result < () , ErrorGuaranteed > { if Some (self . trait_def_id) == trait_def_id { f (self) } else { Ok (()) } } }
    };
}

impl_170!()