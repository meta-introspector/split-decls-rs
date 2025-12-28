macro_rules! remove_graphql_attrs {
    () => {
        pub fn remove_graphql_attrs (attrs : & mut Vec < Attribute >) { if let Some ((idx , _)) = attrs . iter () . enumerate () . find (| (_ , a) | a . path () . is_ident ("graphql")) { attrs . remove (idx) ; } }
    };
}

remove_graphql_attrs!();