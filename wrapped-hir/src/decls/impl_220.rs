macro_rules! deps {
    () => {
        Trait!();
        Function!();
        TypeAlias!();
        AssocItem!();
        Const!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Trait { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write_trait_header (self , f) ? ; let def_id = GenericDefId :: TraitId (self . id) ; let has_where_clause = write_where_clause (def_id , f) ? ; if let Some (limit) = f . entity_limit { let assoc_items = self . items (f . db) ; let count = assoc_items . len () . min (limit) ; f . write_char (if ! has_where_clause { ' ' } else { '\n' }) ? ; if count == 0 { if assoc_items . is_empty () { f . write_str ("{}") ? ; } else { f . write_str ("{ /* … */ }") ? ; } } else { f . write_str ("{\n") ? ; for item in & assoc_items [.. count] { f . write_str ("    ") ? ; match item { AssocItem :: Function (func) => func . hir_fmt (f) , AssocItem :: Const (cst) => cst . hir_fmt (f) , AssocItem :: TypeAlias (type_alias) => type_alias . hir_fmt (f) , } ? ; f . write_str (";\n") ? ; } if assoc_items . len () > count { f . write_str ("    /* … */\n") ? ; } f . write_str ("}") ? ; } } Ok (()) } }
    };
}

impl_220!();