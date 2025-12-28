macro_rules! deps {
    () => {
        DemangleContext!();
        CvQualifiers!();
        DemangleWrite!();
        Result!();
        ArgScopeStack!();
        DemangleNodeType!();
        Demangle!();
        NestedName!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for NestedName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { NestedName :: Unqualified (_ , _ , ref p , ref name) | NestedName :: UnqualifiedExplicitObject (ref p , ref name , _) => { ctx . push_demangle_node (DemangleNodeType :: NestedName) ; if let Some (p) = p . as_ref () { p . demangle (ctx , scope) ? ; ctx . write_str ("::") ? ; } name . demangle (ctx , scope) ? ; ctx . pop_demangle_node () ; } NestedName :: Template (_ , _ , ref p) | NestedName :: TemplateExplicitObject (ref p , _) => { ctx . is_template_prefix_in_nested_name = true ; p . demangle (ctx , scope) ? ; ctx . is_template_prefix_in_nested_name = false ; } } if self . has_explicit_obj_param () { ctx . is_explicit_obj_param = true ; } if let Some (inner) = ctx . pop_inner () { inner . demangle_as_inner (ctx , scope) ? ; } if let Some (cv_qualifiers) = self . cv_qualifiers () { if cv_qualifiers != & CvQualifiers :: default () && ctx . show_params { cv_qualifiers . demangle (ctx , scope) ? ; } } if let Some (ref refs) = self . ref_qualifier () { ctx . ensure_space () ? ; refs . demangle (ctx , scope) ? ; } Ok (()) } }
    };
}

impl_97!();