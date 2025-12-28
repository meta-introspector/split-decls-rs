macro_rules! deps {
    () => {
        EnumVariantCoOccurrenceVisitor!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl < 'a , 'ast > Visit < 'ast > for EnumVariantCoOccurrenceVisitor < 'a > { fn visit_expr_match (& mut self , i : & 'ast syn :: ExprMatch) { let mut matched_variant_types = BTreeSet :: new () ; for arm in & i . arms { if let syn :: Pat :: TupleStruct (pat_tuple_struct) = & arm . pat { if let Some (segment) = pat_tuple_struct . path . segments . last () { matched_variant_types . insert (segment . ident . to_string ()) ; } } else if let syn :: Pat :: Path (pat_path) = & arm . pat { if let Some (segment) = pat_path . path . segments . last () { matched_variant_types . insert (segment . ident . to_string ()) ; } } } if ! matched_variant_types . is_empty () { self . enum_lattice_info . add_co_occurrence (matched_variant_types) ; } visit :: visit_expr_match (self , i) ; } fn visit_expr_if (& mut self , i : & 'ast syn :: ExprIf) { if let syn :: Expr :: Let (expr_let) = & * i . cond { if let syn :: Pat :: TupleStruct (pat_tuple_struct) = & * expr_let . pat { if let Some (segment) = pat_tuple_struct . path . segments . last () { self . enum_lattice_info . add_co_occurrence (BTreeSet :: from ([segment . ident . to_string ()])) ; } } else if let syn :: Pat :: Path (pat_path) = & * expr_let . pat { if let Some (segment) = pat_path . path . segments . last () { self . enum_lattice_info . add_co_occurrence (BTreeSet :: from ([segment . ident . to_string ()])) ; } } } visit :: visit_expr_if (self , i) ; } }
    };
}

impl_203!();