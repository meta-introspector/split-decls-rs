macro_rules! deps {
    () => {
        HirFormatter!();
        HirDisplayError!();
        HirDisplay!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for TraitRef < 'db > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { let trait_ = self . def_id . 0 ; f . start_location_link (trait_ . into ()) ; write ! (f , "{}" , f . db . trait_signature (trait_) . name . display (f . db , f . edition ())) ? ; f . end_location_link () ; let substs = self . args . as_slice () ; hir_fmt_generic_args (f , & substs [1 ..] , None , Some (self . self_ty ())) } }
    };
}

impl_517!()