macro_rules! deps {
    () => {
        GeneratorResult!();
    };
}

macro_rules! parse_graphql_attrs {
    () => {
        deps!();
        pub fn parse_graphql_attrs < T : FromMeta + Default > (attrs : & [Attribute] ,) -> GeneratorResult < Option < T > > { for attr in attrs { if attr . path () . is_ident ("graphql") { return Ok (Some (T :: from_meta (& attr . meta) ?)) ; } } Ok (None) }
    };
}

parse_graphql_attrs!()