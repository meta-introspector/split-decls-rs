mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: { Expr , Stmt } ;}
mkuse!{use rustc_middle :: ty :: { Mutability , TyKind } ;}
mkuse!{use rustc_session :: lint :: FutureIncompatibilityReason ;}
mkuse!{use rustc_session :: { declare_lint , declare_lint_pass } ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: { BytePos , Span } ;}
mkuse!{use crate :: lints :: { MutRefSugg , RefOfMutStatic } ;}
mkuse!{use crate :: { LateContext , LateLintPass , LintContext } ;}
mkitem!{declare_lint ! { # [doc = " The `static_mut_refs` lint checks for shared or mutable references"] # [doc = " of mutable static inside `unsafe` blocks and `unsafe` functions."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2021"] # [doc = " fn main() {"] # [doc = "     static mut X: i32 = 23;"] # [doc = "     static mut Y: i32 = 24;"] # [doc = ""] # [doc = "     unsafe {"] # [doc = "         let y = &X;"] # [doc = "         let ref x = X;"] # [doc = "         let (x, y) = (&X, &Y);"] # [doc = "         foo(&X);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " unsafe fn _foo() {"] # [doc = "     static mut X: i32 = 23;"] # [doc = "     static mut Y: i32 = 24;"] # [doc = ""] # [doc = "     let y = &X;"] # [doc = "     let ref x = X;"] # [doc = "     let (x, y) = (&X, &Y);"] # [doc = "     foo(&X);"] # [doc = " }"] # [doc = ""] # [doc = " fn foo<'a>(_x: &'a i32) {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Shared or mutable references of mutable static are almost always a mistake and"] # [doc = " can lead to undefined behavior and various other problems in your code."] # [doc = ""] # [doc = " This lint is \"warn\" by default on editions up to 2021, in 2024 is \"deny\"."] pub STATIC_MUT_REFS , Warn , "creating a shared reference to mutable static" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionError (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>" , explain_reason : false , } ; @ edition Edition2024 => Deny ; }}
mkitem!{declare_lint_pass ! (StaticMutRefs => [STATIC_MUT_REFS]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for StaticMutRefs { # [allow (rustc :: usage_of_ty_tykind)] fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < '_ >) { let err_span = expr . span ; match expr . kind { hir :: ExprKind :: AddrOf (borrow_kind , m , ex) if matches ! (borrow_kind , hir :: BorrowKind :: Ref) && let Some (err_span) = path_is_static_mut (ex , err_span) => { let source_map = cx . sess () . source_map () ; let snippet = source_map . span_to_snippet (err_span) ; let sugg_span = if let Ok (snippet) = snippet { let exclude_n_bytes : u32 = snippet . chars () . take_while (| ch | ch . is_whitespace () || * ch == '(') . map (| ch | ch . len_utf8 () as u32) . sum () ; err_span . with_lo (err_span . lo () + BytePos (exclude_n_bytes)) . with_hi (ex . span . lo ()) } else { err_span . with_hi (ex . span . lo ()) } ; emit_static_mut_refs (cx , err_span , sugg_span , m , ! expr . span . from_expansion ()) ; } hir :: ExprKind :: MethodCall (_ , e , _ , _) if let Some (err_span) = path_is_static_mut (e , expr . span) && let typeck = cx . typeck_results () && let Some (method_def_id) = typeck . type_dependent_def_id (expr . hir_id) && let inputs = cx . tcx . fn_sig (method_def_id) . skip_binder () . inputs () . skip_binder () && let Some (receiver) = inputs . get (0) && let TyKind :: Ref (_ , _ , m) = receiver . kind () => { emit_static_mut_refs (cx , err_span , err_span . shrink_to_lo () , * m , false) ; } _ => { } } } fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & Stmt < '_ >) { if let hir :: StmtKind :: Let (loc) = stmt . kind && let hir :: PatKind :: Binding (ba , _ , _ , _) = loc . pat . kind && let hir :: ByRef :: Yes (m) = ba . 0 && let Some (init) = loc . init && let Some (err_span) = path_is_static_mut (init , init . span) { emit_static_mut_refs (cx , err_span , err_span . shrink_to_lo () , m , false) ; } } }}}

macro_rules! path_is_static_mut_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_is_static_mut in module {}", module_path!());
    };
}

mkfn!{
    path_is_static_mut_introspect!();
    fn path_is_static_mut (mut expr : & hir :: Expr < '_ > , mut err_span : Span) -> Option < Span > { if err_span . from_expansion () { err_span = expr . span ; } while let hir :: ExprKind :: Field (e , _) = expr . kind { expr = e ; } if let hir :: ExprKind :: Path (qpath) = expr . kind && let hir :: QPath :: Resolved (_ , path) = qpath && let hir :: def :: Res :: Def (def_kind , _) = path . res && let hir :: def :: DefKind :: Static { safety : _ , mutability : Mutability :: Mut , nested : false } = def_kind { return Some (err_span) ; } None }
}

macro_rules! emit_static_mut_refs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_static_mut_refs in module {}", module_path!());
    };
}

mkfn!{
    emit_static_mut_refs_introspect!();
    fn emit_static_mut_refs (cx : & LateContext < '_ > , span : Span , sugg_span : Span , mutable : Mutability , suggest_addr_of : bool ,) { let (shared_label , shared_note , mut_note , sugg) = match mutable { Mutability :: Mut => { let sugg = if suggest_addr_of { Some (MutRefSugg :: Mut { span : sugg_span }) } else { None } ; ("mutable " , false , true , sugg) } Mutability :: Not => { let sugg = if suggest_addr_of { Some (MutRefSugg :: Shared { span : sugg_span }) } else { None } ; ("shared " , true , false , sugg) } } ; cx . emit_span_lint (STATIC_MUT_REFS , span , RefOfMutStatic { span , sugg , shared_label , shared_note , mut_note } ,) ; }
}