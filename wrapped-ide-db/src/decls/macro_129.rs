macro_rules! deps {
    () => {
        Documentation!();
        SymbolKind!();
    };
}

macro_rules! macro_129 {
    () => {
        deps!();
        impl_empty_upmap_from_ra_fixture ! (bool , i8 , i16 , i32 , i64 , i128 , u8 , u16 , u32 , u64 , u128 , f32 , f64 , & str , String , Symbol , SmolStr , Documentation , SymbolKind , CfgExpr , ReferenceCategory ,) ;
    };
}

macro_129!();