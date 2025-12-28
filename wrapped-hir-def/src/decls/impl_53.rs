macro_rules! impl_53 {
    () => {
        impl EnumId { # [inline] pub fn enum_variants (self , db : & dyn DefDatabase) -> & EnumVariants { & self . enum_variants_with_diagnostics (db) . 0 } # [inline] pub fn enum_variants_with_diagnostics (self , db : & dyn DefDatabase ,) -> & (EnumVariants , Option < ThinVec < InactiveEnumVariantCode > >) { EnumVariants :: of (db , self) } }
    };
}

impl_53!()