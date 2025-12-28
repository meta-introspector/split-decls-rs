macro_rules! deps {
    () => {
        BuiltinUnstableFeatures!();
        LateContext!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for UnstableFeatures { fn check_attribute (& mut self , cx : & LateContext < '_ > , attr : & hir :: Attribute) { if attr . has_name (sym :: feature) && let Some (items) = attr . meta_item_list () { for item in items { cx . emit_span_lint (UNSTABLE_FEATURES , item . span () , BuiltinUnstableFeatures) ; } } } }
    };
}

impl_56!();