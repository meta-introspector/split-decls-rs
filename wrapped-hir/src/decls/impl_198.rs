macro_rules! deps {
    () => {
        Enum!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Enum { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write_visibility (self . module (f . db) . id , self . visibility (f . db) , f) ? ; f . write_str ("enum ") ? ; write ! (f , "{}" , self . name (f . db) . display (f . db , f . edition ())) ? ; let def_id = GenericDefId :: AdtId (AdtId :: EnumId (self . id)) ; write_generic_params (def_id , f) ? ; let has_where_clause = write_where_clause (def_id , f) ? ; if let Some (limit) = f . entity_limit { write_variants (& self . variants (f . db) , has_where_clause , limit , f) ? ; } Ok (()) } }
    };
}

impl_198!()