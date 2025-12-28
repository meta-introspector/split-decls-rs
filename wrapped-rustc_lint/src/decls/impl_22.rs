macro_rules! deps {
    () => {
        LateContext!();
        BuiltinNonShorthandFieldPatterns!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for NonShorthandFieldPatterns { fn check_pat (& mut self , cx : & LateContext < '_ > , pat : & hir :: Pat < '_ >) { if let PatKind :: Struct (ref qpath , field_pats , _) = pat . kind { let variant = cx . typeck_results () . pat_ty (pat) . ty_adt_def () . expect ("struct pattern type is not an ADT") . variant_of_res (cx . qpath_res (qpath , pat . hir_id)) ; for fieldpat in field_pats { if fieldpat . is_shorthand { continue ; } if fieldpat . span . from_expansion () { continue ; } if let PatKind :: Binding (binding_annot , _ , ident , None) = fieldpat . pat . kind { if cx . tcx . find_field_index (ident , variant) == Some (cx . typeck_results () . field_index (fieldpat . hir_id)) { cx . emit_span_lint (NON_SHORTHAND_FIELD_PATTERNS , fieldpat . span , BuiltinNonShorthandFieldPatterns { ident , suggestion : fieldpat . span , prefix : binding_annot . prefix_str () , } ,) ; } } } } } }
    };
}

impl_22!();