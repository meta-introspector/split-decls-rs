macro_rules! deps {
    () => {
        PatternKind!();
        LateContext!();
        InvalidReferenceCastingDiag!();
    };
}

macro_rules! impl_762 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for InvalidReferenceCasting { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let Some ((e , pat)) = borrow_or_assign (cx , expr) { let init = cx . expr_or_init (e) ; let orig_cast = if init . span != e . span { Some (init . span) } else { None } ; let mut peel_casts = { let mut peel_casts_cache = None ; move | | * peel_casts_cache . get_or_insert_with (| | peel_casts (cx , init)) } ; if matches ! (pat , PatternKind :: Borrow { mutbl : Mutability :: Mut } | PatternKind :: Assign) && let Some (ty_has_interior_mutability) = is_cast_from_ref_to_mut_ptr (cx , init , & mut peel_casts) { cx . emit_span_lint (INVALID_REFERENCE_CASTING , expr . span , if pat == PatternKind :: Assign { InvalidReferenceCastingDiag :: AssignToRef { orig_cast , ty_has_interior_mutability , } } else { InvalidReferenceCastingDiag :: BorrowAsMut { orig_cast , ty_has_interior_mutability , } } ,) ; } if let Some ((from_ty_layout , to_ty_layout , e_alloc)) = is_cast_to_bigger_memory_layout (cx , init , & mut peel_casts) { cx . emit_span_lint (INVALID_REFERENCE_CASTING , expr . span , InvalidReferenceCastingDiag :: BiggerLayout { orig_cast , alloc : e_alloc . span , from_ty : from_ty_layout . ty , from_size : from_ty_layout . layout . size () . bytes () , to_ty : to_ty_layout . ty , to_size : to_ty_layout . layout . size () . bytes () , } ,) ; } } } }
    };
}

impl_762!()