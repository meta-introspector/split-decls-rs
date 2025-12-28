macro_rules! deps {
    () => {
        Layout!();
        HirDatabase!();
        TraitEnvironment!();
        LayoutError!();
        LayoutCx!();
    };
}

macro_rules! layout_of_simd_ty {
    () => {
        deps!();
        fn layout_of_simd_ty < 'db > (db : & 'db dyn HirDatabase , id : StructId , repr_packed : bool , args : & GenericArgs < 'db > , env : Arc < TraitEnvironment < 'db > > , dl : & TargetDataLayout ,) -> Result < Arc < Layout > , LayoutError > { let fields = db . field_types (id . into ()) ; let mut fields = fields . iter () ; let Some (TyKind :: Array (e_ty , e_len)) = fields . next () . filter (| _ | fields . next () . is_none ()) . map (| f | (* f . 1) . instantiate (DbInterner :: new_with (db , None , None) , args) . kind ()) else { return Err (LayoutError :: InvalidSimdType) ; } ; let e_len = try_const_usize (db , e_len) . ok_or (LayoutError :: HasErrorConst) ? as u64 ; let e_ly = db . layout_of_ty (e_ty , env) ? ; let cx = LayoutCx :: new (dl) ; Ok (Arc :: new (cx . calc . simd_type (e_ly , e_len , repr_packed) ?)) }
    };
}

layout_of_simd_ty!()