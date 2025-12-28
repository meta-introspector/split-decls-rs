macro_rules! is_use_decl_module {
    () => {
        # [macro_export] macro_rules ! is_use_decl_module { ($ item_use : expr) => { if let syn :: UseTree :: Path (use_tree_path) = &$ item_use . tree { if use_tree_path . ident == "introspector_decl2_macros" { if let syn :: UseTree :: Name (use_tree_name) = &* use_tree_path . tree { use_tree_name . ident == "decl_module" } else { false } } else { false } } else { false } } ; }
    };
}

is_use_decl_module!();