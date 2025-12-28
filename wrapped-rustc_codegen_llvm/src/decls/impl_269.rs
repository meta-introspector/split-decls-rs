macro_rules! deps {
    () => {
        CguCoverageContext!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < 'll , 'tcx > CguCoverageContext < 'll , 'tcx > { pub (crate) fn new () -> Self { Self { pgo_func_name_var_map : Default :: default () , covfun_section_name : Default :: default () } } # [doc = " Returns the list of instances considered \"used\" in this CGU, as"] # [doc = " inferred from the keys of `pgo_func_name_var_map`."] pub (crate) fn instances_used (& self) -> Vec < Instance < 'tcx > > { self . pgo_func_name_var_map . borrow () . keys () . copied () . collect :: < Vec < _ > > () } }
    };
}

impl_269!()