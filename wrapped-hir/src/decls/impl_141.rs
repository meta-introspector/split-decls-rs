macro_rules! deps {
    () => {
        BuiltinAttr!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl BuiltinAttr { fn builtin (name : & str) -> Option < Self > { hir_expand :: inert_attr_macro :: find_builtin_attr_idx (& Symbol :: intern (name)) . map (| idx | BuiltinAttr { idx : idx as u32 }) } pub fn name (& self) -> Name { Name :: new_symbol_root (Symbol :: intern (hir_expand :: inert_attr_macro :: INERT_ATTRIBUTES [self . idx as usize] . name ,)) } pub fn template (& self) -> Option < AttributeTemplate > { Some (hir_expand :: inert_attr_macro :: INERT_ATTRIBUTES [self . idx as usize] . template) } }
    };
}

impl_141!()