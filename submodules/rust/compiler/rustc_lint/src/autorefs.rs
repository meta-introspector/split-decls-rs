mkuse!{use rustc_ast :: { BorrowKind , UnOp } ;}
mkuse!{use rustc_hir :: { Expr , ExprKind , Mutability } ;}
mkuse!{use rustc_middle :: ty :: adjustment :: { Adjust , Adjustment , AutoBorrow , OverloadedDeref } ;}
mkuse!{use rustc_session :: { declare_lint , declare_lint_pass } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use crate :: lints :: { ImplicitUnsafeAutorefsDiag , ImplicitUnsafeAutorefsMethodNote , ImplicitUnsafeAutorefsOrigin , ImplicitUnsafeAutorefsSuggestion , } ;}
mkuse!{use crate :: { LateContext , LateLintPass , LintContext } ;}
mkitem!{declare_lint ! { # [doc = " The `dangerous_implicit_autorefs` lint checks for implicitly taken references"] # [doc = " to dereferences of raw pointers."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " unsafe fn fun(ptr: *mut [u8]) -> *mut [u8] {"] # [doc = "     unsafe { &raw mut (*ptr)[..16] }"] # [doc = "     //                      ^^^^^^ this calls `IndexMut::index_mut(&mut ..., ..16)`,"] # [doc = "     //                             implicitly creating a reference"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " When working with raw pointers it's usually undesirable to create references,"] # [doc = " since they inflict additional safety requirements. Unfortunately, it's possible"] # [doc = " to take a reference to the dereference of a raw pointer implicitly, which inflicts"] # [doc = " the usual reference requirements."] # [doc = ""] # [doc = " If you are sure that you can soundly take a reference, then you can take it explicitly:"] # [doc = ""] # [doc = " ```rust"] # [doc = " unsafe fn fun(ptr: *mut [u8]) -> *mut [u8] {"] # [doc = "     unsafe { &raw mut (&mut *ptr)[..16] }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Otherwise try to find an alternative way to achieve your goals using only raw pointers:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::ptr;"] # [doc = ""] # [doc = " fn fun(ptr: *mut [u8]) -> *mut [u8] {"] # [doc = "     ptr::slice_from_raw_parts_mut(ptr.cast(), 16)"] # [doc = " }"] # [doc = " ```"] pub DANGEROUS_IMPLICIT_AUTOREFS , Deny , "implicit reference to a dereference of a raw pointer" , report_in_external_macro }}
mkitem!{declare_lint_pass ! (ImplicitAutorefs => [DANGEROUS_IMPLICIT_AUTOREFS]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for ImplicitAutorefs { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let mut is_coming_from_deref = false ; let inner = match expr . kind { ExprKind :: AddrOf (BorrowKind :: Raw , _ , inner) => match inner . kind { ExprKind :: Unary (UnOp :: Deref , inner) => { is_coming_from_deref = true ; inner } _ => return , } , ExprKind :: Index (base , _ , _) => base , ExprKind :: MethodCall (_ , inner , _ , _) => { inner } ExprKind :: Field (inner , _) => inner , _ => return , } ; let typeck = cx . typeck_results () ; let adjustments_table = typeck . adjustments () ; if let Some (adjustments) = adjustments_table . get (inner . hir_id) && let adjustments = peel_derefs_adjustments (& * * adjustments) && let [adjustment] = adjustments && let Some ((borrow_mutbl , through_overloaded_deref)) = has_implicit_borrow (adjustment) && let ExprKind :: Unary (UnOp :: Deref , dereferenced) = peel_place_mappers (inner) . kind && typeck . expr_ty (dereferenced) . is_raw_ptr () && let method_did = match expr . kind { ExprKind :: MethodCall (..) => cx . typeck_results () . type_dependent_def_id (expr . hir_id) , _ => None , } && method_did . map (| did | cx . tcx . has_attr (did , sym :: rustc_no_implicit_autorefs)) . unwrap_or (true) { cx . emit_span_lint (DANGEROUS_IMPLICIT_AUTOREFS , expr . span . source_callsite () , ImplicitUnsafeAutorefsDiag { raw_ptr_span : dereferenced . span , raw_ptr_ty : typeck . expr_ty (dereferenced) , origin : if through_overloaded_deref { ImplicitUnsafeAutorefsOrigin :: OverloadedDeref } else { ImplicitUnsafeAutorefsOrigin :: Autoref { autoref_span : inner . span , autoref_ty : typeck . expr_ty_adjusted (inner) , } } , method : method_did . map (| did | ImplicitUnsafeAutorefsMethodNote { def_span : cx . tcx . def_span (did) , method_name : cx . tcx . item_name (did) , }) , suggestion : ImplicitUnsafeAutorefsSuggestion { mutbl : borrow_mutbl . ref_prefix_str () , deref : if is_coming_from_deref { "*" } else { "" } , start_span : inner . span . shrink_to_lo () , end_span : inner . span . shrink_to_hi () , } , } ,) } } }}}

macro_rules! peel_place_mappers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function peel_place_mappers in module {}", module_path!());
    };
}

mkfn!{
    peel_place_mappers_introspect!();
    # [doc = " Peels expressions from `expr` that can map a place."] fn peel_place_mappers < 'tcx > (mut expr : & 'tcx Expr < 'tcx >) -> & 'tcx Expr < 'tcx > { loop { match expr . kind { ExprKind :: Index (base , _idx , _) => expr = & base , ExprKind :: Field (e , _) => expr = & e , _ => break expr , } } }
}

macro_rules! peel_derefs_adjustments_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function peel_derefs_adjustments in module {}", module_path!());
    };
}

mkfn!{
    peel_derefs_adjustments_introspect!();
    # [doc = " Peel derefs adjustments until the last last element."] fn peel_derefs_adjustments < 'a > (mut adjs : & 'a [Adjustment < 'a >]) -> & 'a [Adjustment < 'a >] { while let [Adjustment { kind : Adjust :: Deref (_) , .. } , end @ ..] = adjs && ! end . is_empty () { adjs = end ; } adjs }
}

macro_rules! has_implicit_borrow_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_implicit_borrow in module {}", module_path!());
    };
}

mkfn!{
    has_implicit_borrow_introspect!();
    # [doc = " Test if some adjustment has some implicit borrow."] # [doc = ""] # [doc = " Returns `Some((mutability, was_an_overloaded_deref))` if the argument adjustment is"] # [doc = " an implicit borrow (or has an implicit borrow via an overloaded deref)."] fn has_implicit_borrow (Adjustment { kind , .. } : & Adjustment < '_ >) -> Option < (Mutability , bool) > { match kind { & Adjust :: Deref (Some (OverloadedDeref { mutbl , .. })) => Some ((mutbl , true)) , & Adjust :: Borrow (AutoBorrow :: Ref (mutbl)) => Some ((mutbl . into () , false)) , Adjust :: NeverToAny | Adjust :: Pointer (..) | Adjust :: ReborrowPin (..) | Adjust :: Deref (None) | Adjust :: Borrow (AutoBorrow :: RawPtr (..)) => None , } }
}