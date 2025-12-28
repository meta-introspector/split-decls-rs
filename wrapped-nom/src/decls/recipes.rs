macro_rules! recipes {
    () => {
        # [cfg (all (feature = "std" , any (doc , doctest , feature = "docsrs")))] # [cfg_attr (any (doc , doctest , feature = "docsrs") , doc = include_str ! ("../doc/nom_recipes.md"))] pub mod recipes { }
    };
}

recipes!();