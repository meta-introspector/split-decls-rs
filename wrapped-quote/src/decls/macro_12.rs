macro_rules! macro_12 {
    () => {
        # [cfg (doc)] __quote_spanned ! [# [macro_export] macro_rules ! quote_spanned { ($ span : expr => $ ($ tt : tt) *) => { ... } ; }] ;
    };
}

macro_12!()