macro_rules! macro_9 {
    () => {
        # [cfg (doc)] __quote ! [# [macro_export] macro_rules ! quote { ($ ($ tt : tt) *) => { ... } ; }] ;
    };
}

macro_9!()