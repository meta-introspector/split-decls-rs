macro_rules! deps {
    () => {
        TypeAlias!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for TypeAlias { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write_visibility (self . module (f . db) . id , self . visibility (f . db) , f) ? ; let data = f . db . type_alias_signature (self . id) ; write ! (f , "type {}" , data . name . display (f . db , f . edition ())) ? ; let def_id = GenericDefId :: TypeAliasId (self . id) ; write_generic_params (def_id , f) ? ; if ! data . bounds . is_empty () { f . write_str (": ") ? ; f . write_joined (data . bounds . iter () . map (| bound | hir_display_with_store (bound , & data . store)) , " + " ,) ? ; } if let Some (ty) = data . ty { f . write_str (" = ") ? ; ty . hir_fmt (f , & data . store) ? ; } write_where_clause (def_id , f) ? ; Ok (()) } }
    };
}

impl_222!();