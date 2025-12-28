macro_rules! deps {
    () => {
        ExpressionStoreSourceMap!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl PartialEq for ExpressionStoreSourceMap { fn eq (& self , other : & Self) -> bool { let Self { expr_only , types_map_back , types_map : _ , lifetime_map_back , lifetime_map : _ } = self ; * expr_only == other . expr_only && * types_map_back == other . types_map_back && * lifetime_map_back == other . lifetime_map_back } }
    };
}

impl_286!();