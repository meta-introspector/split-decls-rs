macro_rules! deps {
    () => {
        ExpressionStoreAdapter!();
        HirDisplayWithExpressionStore!();
        HirDisplay!();
    };
}

macro_rules! hir_display_with_store {
    () => {
        deps!();
        pub fn hir_display_with_store < 'a , 'db , T : HirDisplayWithExpressionStore < 'db > + 'a > (value : T , store : & 'a ExpressionStore ,) -> impl HirDisplay < 'db > + 'a { ExpressionStoreAdapter (value , store) }
    };
}

hir_display_with_store!();