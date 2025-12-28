macro_rules! deps {
    () => {
        Impl!();
    };
}

macro_rules! write_impl_header {
    () => {
        deps!();
        fn write_impl_header < 'db > (impl_ : & Impl , f : & mut HirFormatter < '_ , 'db > ,) -> Result < () , HirDisplayError > { let db = f . db ; f . write_str ("impl") ? ; let def_id = GenericDefId :: ImplId (impl_ . id) ; write_generic_params (def_id , f) ? ; if let Some (trait_) = impl_ . trait_ (db) { let trait_data = db . trait_signature (trait_ . id) ; write ! (f , " {} for" , trait_data . name . display (db , f . edition ())) ? ; } f . write_char (' ') ? ; impl_ . self_ty (db) . hir_fmt (f) ? ; Ok (()) }
    };
}

write_impl_header!()