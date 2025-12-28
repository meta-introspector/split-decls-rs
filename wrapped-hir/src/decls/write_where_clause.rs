macro_rules! write_where_clause {
    () => {
        fn write_where_clause < 'db > (def : GenericDefId , f : & mut HirFormatter < '_ , 'db > ,) -> Result < bool , HirDisplayError > { let (params , store) = f . db . generic_params_and_store (def) ; if ! has_disaplayable_predicates (f . db , & params , & store) { return Ok (false) ; } f . write_str ("\nwhere") ? ; write_where_predicates (& params , & store , f) ? ; Ok (true) }
    };
}

write_where_clause!();