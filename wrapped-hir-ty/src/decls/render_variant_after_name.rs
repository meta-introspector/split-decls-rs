macro_rules! deps {
    () => {
        HirDisplayError!();
        EarlyBinder!();
        HirFormatter!();
        MemoryMap!();
        Layout!();
        TraitEnvironment!();
    };
}

macro_rules! render_variant_after_name {
    () => {
        deps!();
        fn render_variant_after_name < 'db > (data : & VariantFields , f : & mut HirFormatter < '_ , 'db > , field_types : & ArenaMap < LocalFieldId , EarlyBinder < 'db , Ty < 'db > > > , trait_env : Arc < TraitEnvironment < 'db > > , layout : & Layout , args : GenericArgs < 'db > , b : & [u8] , memory_map : & MemoryMap < 'db > ,) -> Result < () , HirDisplayError > { match data . shape { FieldsShape :: Record | FieldsShape :: Tuple => { let render_field = | f : & mut HirFormatter < '_ , 'db > , id : LocalFieldId | { let offset = layout . fields . offset (u32 :: from (id . into_raw ()) as usize) . bytes_usize () ; let ty = field_types [id] . instantiate (f . interner , args) ; let Ok (layout) = f . db . layout_of_ty (ty , trait_env . clone ()) else { return f . write_str ("<layout-error>") ; } ; let size = layout . size . bytes_usize () ; render_const_scalar (f , & b [offset .. offset + size] , memory_map , ty) } ; let mut it = data . fields () . iter () ; if matches ! (data . shape , FieldsShape :: Record) { write ! (f , " {{") ? ; if let Some ((id , data)) = it . next () { write ! (f , " {}: " , data . name . display (f . db , f . edition ())) ? ; render_field (f , id) ? ; } for (id , data) in it { write ! (f , ", {}: " , data . name . display (f . db , f . edition ())) ? ; render_field (f , id) ? ; } write ! (f , " }}") ? ; } else { let mut it = it . map (| it | it . 0) ; write ! (f , "(") ? ; if let Some (id) = it . next () { render_field (f , id) ? ; } for id in it { write ! (f , ", ") ? ; render_field (f , id) ? ; } write ! (f , ")") ? ; } Ok (()) } FieldsShape :: Unit => Ok (()) , } }
    };
}

render_variant_after_name!();