macro_rules! wrap_either {
    () => {
        macro_rules ! wrap_either { ($ value : expr => $ ($ tail : tt) *) => { match $ value { Left (inner) => inner . map (Left) $ ($ tail) *, Right (inner) => inner . map (Right) $ ($ tail) *, } } ; }
    };
}

wrap_either!()