macro_rules! map_either {
    () => {
        macro_rules ! map_either { ($ value : expr , $ pattern : pat => $ result : expr) => { match $ value { Left ($ pattern) => Left ($ result) , Right ($ pattern) => Right ($ result) , } } ; }
    };
}

map_either!();