macro_rules! KEY_COMPLEXITY {
    () => {
        const KEY_COMPLEXITY : Key = Key :: from_static_str ("graphql.complexity") ;
    };
}

KEY_COMPLEXITY!()