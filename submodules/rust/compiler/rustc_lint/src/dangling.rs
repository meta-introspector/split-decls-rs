mkuse!{use rustc_ast :: visit :: { visit_opt , walk_list } ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: Res ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: intravisit :: { FnKind , Visitor , walk_expr } ;}
mkuse!{use rustc_hir :: { Block , Body , Expr , ExprKind , FnDecl , FnRetTy , LangItem , TyKind , find_attr } ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: { declare_lint , impl_lint_pass } ;}
mkuse!{use rustc_span :: { Span , sym } ;}
mkuse!{use crate :: lints :: { DanglingPointersFromLocals , DanglingPointersFromTemporaries } ;}
mkuse!{use crate :: { LateContext , LateLintPass } ;}
mkitem!{declare_lint ! { # [doc = " The `dangling_pointers_from_temporaries` lint detects getting a pointer to data"] # [doc = " of a temporary that will immediately get dropped."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #![allow(unused)]"] # [doc = " # unsafe fn use_data(ptr: *const u8) { }"] # [doc = " fn gather_and_use(bytes: impl Iterator<Item = u8>) {"] # [doc = "     let x: *const u8 = bytes.collect::<Vec<u8>>().as_ptr();"] # [doc = "     unsafe { use_data(x) }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Getting a pointer from a temporary value will not prolong its lifetime,"] # [doc = " which means that the value can be dropped and the allocation freed"] # [doc = " while the pointer still exists, making the pointer dangling."] # [doc = " This is not an error (as far as the type system is concerned)"] # [doc = " but probably is not what the user intended either."] # [doc = ""] # [doc = " If you need stronger guarantees, consider using references instead,"] # [doc = " as they are statically verified by the borrow-checker to never dangle."] pub DANGLING_POINTERS_FROM_TEMPORARIES , Warn , "detects getting a pointer from a temporary" }}
mkitem!{declare_lint ! { # [doc = " The `dangling_pointers_from_locals` lint detects getting a pointer to data"] # [doc = " of a local that will be dropped at the end of the function."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn f() -> *const u8 {"] # [doc = "     let x = 0;"] # [doc = "     &x // returns a dangling ptr to `x`"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Returning a pointer from a local value will not prolong its lifetime,"] # [doc = " which means that the value can be dropped and the allocation freed"] # [doc = " while the pointer still exists, making the pointer dangling."] # [doc = " This is not an error (as far as the type system is concerned)"] # [doc = " but probably is not what the user intended either."] # [doc = ""] # [doc = " If you need stronger guarantees, consider using references instead,"] # [doc = " as they are statically verified by the borrow-checker to never dangle."] pub DANGLING_POINTERS_FROM_LOCALS , Warn , "detects returning a pointer from a local variable" }}
mkitem!{mkstruct!{# [doc = " FIXME: false negatives (i.e. the lint is not emitted when it should be)"] # [doc = " 1. Ways to get a temporary that are not recognized:"] # [doc = "    - `owning_temporary.field`"] # [doc = "    - `owning_temporary[index]`"] # [doc = " 2. No checks for ref-to-ptr conversions:"] # [doc = "    - `&raw [mut] temporary`"] # [doc = "    - `&temporary as *(const|mut) _`"] # [doc = "    - `ptr::from_ref(&temporary)` and friends"] # [derive (Clone , Copy , Default)] pub (crate) struct DanglingPointers ;}}
mkitem!{impl_lint_pass ! (DanglingPointers => [DANGLING_POINTERS_FROM_TEMPORARIES , DANGLING_POINTERS_FROM_LOCALS]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for DanglingPointers { fn check_fn (& mut self , cx : & LateContext < 'tcx > , fn_kind : FnKind < 'tcx > , fn_decl : & 'tcx FnDecl < 'tcx > , body : & 'tcx Body < 'tcx > , _ : Span , def_id : LocalDefId ,) { DanglingPointerSearcher { cx , inside_call_args : false } . visit_body (body) ; if let FnRetTy :: Return (ret_ty) = & fn_decl . output && let TyKind :: Ptr (_) = ret_ty . kind { let ty = match cx . tcx . type_of (def_id) . instantiate_identity () . kind () { ty :: FnDef (..) => cx . tcx . fn_sig (def_id) . instantiate_identity () , ty :: Closure (_ , args) => args . as_closure () . sig () , _ => return , } ; let ty = ty . output () ; let ty = cx . tcx . instantiate_bound_regions_with_erased (ty) ; let inner_ty = match ty . kind () { ty :: RawPtr (inner_ty , _) => * inner_ty , _ => return , } ; if cx . tcx . layout_of (cx . typing_env () . as_query_input (inner_ty)) . is_ok_and (| layout | ! layout . is_1zst ()) { let dcx = & DanglingPointerLocalContext { body : def_id , fn_ret : ty , fn_ret_span : ret_ty . span , fn_ret_inner : inner_ty , fn_kind : match fn_kind { FnKind :: ItemFn (..) => "function" , FnKind :: Method (..) => "method" , FnKind :: Closure => "closure" , } , } ; DanglingPointerReturnSearcher { cx , dcx } . visit_body (body) ; if let ExprKind :: Block (block , None) = & body . value . kind && let innermost_block = block . innermost_block () && let Some (expr) = innermost_block . expr { lint_addr_of_local (cx , dcx , expr) ; } } } } }}}
mkitem!{mkstruct!{struct DanglingPointerLocalContext < 'tcx > { body : LocalDefId , fn_ret : Ty < 'tcx > , fn_ret_span : Span , fn_ret_inner : Ty < 'tcx > , fn_kind : & 'static str , }}}
mkitem!{mkstruct!{struct DanglingPointerReturnSearcher < 'lcx , 'tcx > { cx : & 'lcx LateContext < 'tcx > , dcx : & 'lcx DanglingPointerLocalContext < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for DanglingPointerReturnSearcher < '_ , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> Self :: Result { if let ExprKind :: Ret (Some (expr)) = expr . kind { lint_addr_of_local (self . cx , self . dcx , expr) ; } walk_expr (self , expr) } }}}

macro_rules! lint_addr_of_local_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_addr_of_local in module {}", module_path!());
    };
}

mkfn!{
    lint_addr_of_local_introspect!();
    # [doc = " Look for `&<path_to_local_in_same_body>` pattern and emit lint for it"] fn lint_addr_of_local < 'a > (cx : & LateContext < 'a > , dcx : & DanglingPointerLocalContext < 'a > , expr : & 'a Expr < 'a > ,) { let (inner , _) = super :: utils :: peel_casts (cx , expr) ; if let ExprKind :: AddrOf (_ , _ , inner_of) = inner . kind && let ExprKind :: Path (ref qpath) = inner_of . peel_blocks () . kind && let Res :: Local (from) = cx . qpath_res (qpath , inner_of . hir_id) && cx . tcx . hir_enclosing_body_owner (from) == dcx . body { cx . tcx . emit_node_span_lint (DANGLING_POINTERS_FROM_LOCALS , expr . hir_id , expr . span , DanglingPointersFromLocals { ret_ty : dcx . fn_ret , ret_ty_span : dcx . fn_ret_span , fn_kind : dcx . fn_kind , local_var : cx . tcx . hir_span (from) , local_var_name : cx . tcx . hir_ident (from) , local_var_ty : dcx . fn_ret_inner , created_at : (expr . hir_id != inner . hir_id) . then_some (inner . span) , } ,) ; } }
}
mkitem!{mkstruct!{# [doc = " This produces a dangling pointer:"] # [doc = " ```ignore (example)"] # [doc = " let ptr = CString::new(\"hello\").unwrap().as_ptr();"] # [doc = " foo(ptr)"] # [doc = " ```"] # [doc = ""] # [doc = " But this does not:"] # [doc = " ```ignore (example)"] # [doc = " foo(CString::new(\"hello\").unwrap().as_ptr())"] # [doc = " ```"] # [doc = ""] # [doc = " But this does:"] # [doc = " ```ignore (example)"] # [doc = " foo({ let ptr = CString::new(\"hello\").unwrap().as_ptr(); ptr })"] # [doc = " ```"] # [doc = ""] # [doc = " So we have to keep track of when we are inside of a function/method call argument."] struct DanglingPointerSearcher < 'lcx , 'tcx > { cx : & 'lcx LateContext < 'tcx > , # [doc = " Keeps track of whether we are inside of function/method call arguments,"] # [doc = " where this lint should not be emitted."] # [doc = ""] # [doc = " See [the main doc][`Self`] for examples."] inside_call_args : bool , }}}
mkitem!{mkimpl!{impl Visitor < '_ > for DanglingPointerSearcher < '_ , '_ > { fn visit_expr (& mut self , expr : & Expr < '_ >) -> Self :: Result { if ! self . inside_call_args { lint_expr (self . cx , expr) } match expr . kind { ExprKind :: Call (lhs , args) | ExprKind :: MethodCall (_ , lhs , args , _) => { self . visit_expr (lhs) ; self . with_inside_call_args (true , | this | walk_list ! (this , visit_expr , args)) } ExprKind :: Block (& Block { stmts , expr , .. } , _) => { self . with_inside_call_args (false , | this | walk_list ! (this , visit_stmt , stmts)) ; visit_opt ! (self , visit_expr , expr) } _ => walk_expr (self , expr) , } } }}}
mkitem!{mkimpl!{impl DanglingPointerSearcher < '_ , '_ > { fn with_inside_call_args < R > (& mut self , inside_call_args : bool , callback : impl FnOnce (& mut Self) -> R ,) -> R { let old = core :: mem :: replace (& mut self . inside_call_args , inside_call_args) ; let result = callback (self) ; self . inside_call_args = old ; result } }}}

macro_rules! lint_expr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lint_expr in module {}", module_path!());
    };
}

mkfn!{
    lint_expr_introspect!();
    fn lint_expr (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: MethodCall (method , receiver , _args , _span) = expr . kind && is_temporary_rvalue (receiver) && let ty = cx . typeck_results () . expr_ty (receiver) && owns_allocation (cx . tcx , ty) && let Some (fn_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && find_attr ! (cx . tcx . get_all_attrs (fn_id) , AttributeKind :: AsPtr (_)) { cx . tcx . emit_node_span_lint (DANGLING_POINTERS_FROM_TEMPORARIES , expr . hir_id , method . ident . span , DanglingPointersFromTemporaries { callee : method . ident , ty , ptr_span : method . ident . span , temporary_span : receiver . span , } ,) } }
}

macro_rules! is_temporary_rvalue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_temporary_rvalue in module {}", module_path!());
    };
}

mkfn!{
    is_temporary_rvalue_introspect!();
    fn is_temporary_rvalue (expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: ConstBlock (..) | ExprKind :: Repeat (..) | ExprKind :: Lit (..) => false , ExprKind :: Path (..) => false , ExprKind :: Call (..) | ExprKind :: MethodCall (..) | ExprKind :: Use (..) | ExprKind :: Binary (..) => true , ExprKind :: If (..) | ExprKind :: Loop (..) | ExprKind :: Match (..) | ExprKind :: Block (..) => true , ExprKind :: Index (..) | ExprKind :: Field (..) | ExprKind :: Unary (..) => false , ExprKind :: Struct (..) => true , ExprKind :: Array (..) => false , ExprKind :: Break (..) | ExprKind :: Continue (..) | ExprKind :: Ret (..) | ExprKind :: Become (..) => { false } ExprKind :: Assign (..) | ExprKind :: AssignOp (..) | ExprKind :: Yield (..) => false , ExprKind :: AddrOf (..) | ExprKind :: OffsetOf (..) | ExprKind :: InlineAsm (..) => false , ExprKind :: Cast (..) | ExprKind :: Closure (..) | ExprKind :: Tup (..) | ExprKind :: DropTemps (..) | ExprKind :: Let (..) => false , ExprKind :: UnsafeBinderCast (..) => false , ExprKind :: Type (..) | ExprKind :: Err (..) => false , } }
}

macro_rules! owns_allocation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function owns_allocation in module {}", module_path!());
    };
}

mkfn!{
    owns_allocation_introspect!();
    fn owns_allocation (tcx : TyCtxt < '_ > , ty : Ty < '_ >) -> bool { if ty . is_array () { true } else if let Some (inner) = ty . boxed_ty () { inner . is_slice () || inner . is_str () || inner . ty_adt_def () . is_some_and (| def | tcx . is_lang_item (def . did () , LangItem :: CStr)) || owns_allocation (tcx , inner) } else if let Some (def) = ty . ty_adt_def () { for lang_item in [LangItem :: String , LangItem :: MaybeUninit , LangItem :: UnsafeCell] { if tcx . is_lang_item (def . did () , lang_item) { return true ; } } tcx . get_diagnostic_name (def . did ()) . is_some_and (| name | { matches ! (name , sym :: cstring_type | sym :: Vec | sym :: Cell | sym :: SyncUnsafeCell) }) } else { false } }
}