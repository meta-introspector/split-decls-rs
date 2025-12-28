macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < 'db > HirDisplay < 'db > for Variant { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { write ! (f , "{}" , self . name (f . db) . display (f . db , f . edition ())) ? ; let data = self . id . fields (f . db) ; match data . shape { FieldsShape :: Unit => { } FieldsShape :: Tuple => { f . write_char ('(') ? ; let mut first = true ; for (_ , field) in data . fields () . iter () { if first { first = false ; } else { f . write_str (", ") ? ; } field . type_ref . hir_fmt (f , & data . store) ? ; } f . write_char (')') ? ; } FieldsShape :: Record => { if let Some (limit) = f . entity_limit { write_fields (& self . fields (f . db) , false , limit , true , f) ? ; } } } Ok (()) } }
    };
}

impl_204!();