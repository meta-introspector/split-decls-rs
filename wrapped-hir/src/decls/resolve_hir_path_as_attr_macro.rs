macro_rules! deps {
    () => {
        Macro!();
    };
}

macro_rules! resolve_hir_path_as_attr_macro {
    () => {
        deps!();
        # [inline] pub (crate) fn resolve_hir_path_as_attr_macro (db : & dyn HirDatabase , resolver : & Resolver < '_ > , path : & Path ,) -> Option < Macro > { resolver . resolve_path_as_macro (db , path . mod_path () ? , Some (MacroSubNs :: Attr)) . map (| (it , _) | it) . map (Into :: into) }
    };
}

resolve_hir_path_as_attr_macro!()