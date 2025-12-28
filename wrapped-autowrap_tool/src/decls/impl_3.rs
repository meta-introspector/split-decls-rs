macro_rules! deps {
    () => {
        ImplCallVisitor!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'ast > syn :: visit :: Visit < 'ast > for ImplCallVisitor { fn visit_macro (& mut self , i : & 'ast syn :: Macro) { if let Some (path_segment) = i . path . segments . last () { let path_str = path_segment . ident . to_string () ; if path_str . ends_with ("_impl") { if let Some (module_ident) = i . path . segments . first () { let module_name = module_ident . ident . to_string () ; let fn_name = path_str ; self . calls . entry (module_name) . or_default () . insert (fn_name) ; } } } syn :: visit :: visit_macro (self , i) ; } }
    };
}

impl_3!()