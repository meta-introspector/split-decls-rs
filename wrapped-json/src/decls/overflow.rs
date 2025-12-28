macro_rules! overflow {
    () => {
        macro_rules ! overflow { ($ a : ident * 10 + $ b : ident , $ c : expr) => { match $ c { c => $ a >= c / 10 && ($ a > c / 10 || $ b > c % 10) , } } ; }
    };
}

overflow!();