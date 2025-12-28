macro_rules! macro_64 {
    () => {
        # [cfg (doc)] __quote_spanned ! [# [macro_export] macro_rules ! quote_spanned { ($ span : expr => $ ($ tt : tt) *) => { ... } ; }] ;
    };
}

macro_64!();