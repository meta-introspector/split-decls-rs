macro_rules! deps {
    () => {
        UnifiedDiff!();
        Token!();
        Diff!();
        UnifiedDiffConfig!();
        UnifiedDiffPrinter!();
        InternedInput!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl Diff { pub fn unified_diff < 'a , P : UnifiedDiffPrinter , T : Hash + Eq > (& 'a self , printer : & 'a P , config : UnifiedDiffConfig , input : & 'a InternedInput < T > ,) -> UnifiedDiff < 'a , P > { self . unified_diff_with (printer , config , & input . before , & input . after) } pub fn unified_diff_with < 'a , P : UnifiedDiffPrinter > (& 'a self , printer : & 'a P , config : UnifiedDiffConfig , before : & 'a [Token] , after : & 'a [Token] ,) -> UnifiedDiff < 'a , P > { UnifiedDiff { printer , diff : self , config , before , after , } } }
    };
}

impl_73!()