mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: limit :: Limit ;}
mkuse!{use rustc_hir :: { Attribute , find_attr } ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_session :: Limits ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { providers . limits = | tcx , () | { let attrs = tcx . hir_krate_attrs () ; Limits { recursion_limit : get_recursion_limit (tcx . hir_krate_attrs ()) , move_size_limit : find_attr ! (attrs , AttributeKind :: MoveSizeLimit { limit , .. } => * limit) . unwrap_or (Limit :: new (tcx . sess . opts . unstable_opts . move_size_limit . unwrap_or (0))) , type_length_limit : find_attr ! (attrs , AttributeKind :: TypeLengthLimit { limit , .. } => * limit) . unwrap_or (Limit :: new (2usize . pow (24))) , pattern_complexity_limit : find_attr ! (attrs , AttributeKind :: PatternComplexityLimit { limit , .. } => * limit) . unwrap_or (Limit :: unlimited ()) , } } }
}

macro_rules! get_recursion_limit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_recursion_limit in module {}", module_path!());
    };
}

mkfn!{
    get_recursion_limit_introspect!();
    pub (crate) fn get_recursion_limit (attrs : & [Attribute]) -> Limit { find_attr ! (attrs , AttributeKind :: RecursionLimit { limit , .. } => * limit) . unwrap_or (Limit :: new (128)) }
}