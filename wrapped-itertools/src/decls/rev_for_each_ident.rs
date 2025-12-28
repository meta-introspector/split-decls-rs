macro_rules! rev_for_each_ident {
    () => {
        macro_rules ! rev_for_each_ident { ($ m : ident ,) => { } ; ($ m : ident , $ i0 : ident , $ ($ i : ident ,) *) => { rev_for_each_ident ! ($ m , $ ($ i ,) *) ; $ m ! ($ i0) ; } ; }
    };
}

rev_for_each_ident!();