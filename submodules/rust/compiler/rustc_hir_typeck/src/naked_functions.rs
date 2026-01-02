mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_hir :: intravisit :: Visitor ;}
mkuse!{use rustc_hir :: { ExprKind , HirIdSet , StmtKind , find_attr } ;}
mkuse!{use rustc_middle :: span_bug ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use crate :: errors :: { NakedFunctionsAsmBlock , NakedFunctionsMustNakedAsm , NoPatterns , ParamsNotAllowed , } ;}

macro_rules! typeck_naked_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function typeck_naked_fn in module {}", module_path!());
    };
}

mkfn!{
    typeck_naked_fn_introspect!();
    # [doc = " Naked fns can only have trivial binding patterns in arguments,"] # [doc = " may not actually use those arguments, and the body must consist of just"] # [doc = " a single asm statement."] pub (crate) fn typeck_naked_fn < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , body : & 'tcx hir :: Body < 'tcx > ,) { debug_assert ! (find_attr ! (tcx . get_all_attrs (def_id) , AttributeKind :: Naked (..))) ; check_no_patterns (tcx , body . params) ; check_no_parameters_use (tcx , body) ; check_asm (tcx , def_id , body) ; }
}

macro_rules! check_no_patterns_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_no_patterns in module {}", module_path!());
    };
}

mkfn!{
    check_no_patterns_introspect!();
    # [doc = " Checks that parameters don't use patterns. Mirrors the checks for function declarations."] fn check_no_patterns (tcx : TyCtxt < '_ > , params : & [hir :: Param < '_ >]) { for param in params { match param . pat . kind { hir :: PatKind :: Wild | hir :: PatKind :: Binding (hir :: BindingMode :: NONE , _ , _ , None) => { } _ => { tcx . dcx () . emit_err (NoPatterns { span : param . pat . span }) ; } } } }
}

macro_rules! check_no_parameters_use_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_no_parameters_use in module {}", module_path!());
    };
}

mkfn!{
    check_no_parameters_use_introspect!();
    # [doc = " Checks that function parameters aren't used in the function body."] fn check_no_parameters_use < 'tcx > (tcx : TyCtxt < 'tcx > , body : & 'tcx hir :: Body < 'tcx >) { let mut params = HirIdSet :: default () ; for param in body . params { param . pat . each_binding (| _binding_mode , hir_id , _span , _ident | { params . insert (hir_id) ; }) ; } CheckParameters { tcx , params } . visit_body (body) ; }
}
mkitem!{mkstruct!{struct CheckParameters < 'tcx > { tcx : TyCtxt < 'tcx > , params : HirIdSet , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for CheckParameters < 'tcx > { fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Path (hir :: QPath :: Resolved (_ , hir :: Path { res : hir :: def :: Res :: Local (var_hir_id) , .. } ,)) = expr . kind { if self . params . contains (var_hir_id) { self . tcx . dcx () . emit_err (ParamsNotAllowed { span : expr . span }) ; return ; } } hir :: intravisit :: walk_expr (self , expr) ; } }}}

macro_rules! check_asm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_asm in module {}", module_path!());
    };
}

mkfn!{
    check_asm_introspect!();
    # [doc = " Checks that function body contains a single inline assembly block."] fn check_asm < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId , body : & 'tcx hir :: Body < 'tcx >) { let mut this = CheckInlineAssembly { items : Vec :: new () } ; this . visit_body (body) ; if let [(ItemKind :: NakedAsm | ItemKind :: Err , _)] = this . items [..] { } else { let mut must_show_error = false ; let mut has_naked_asm = false ; let mut has_err = false ; let mut multiple_asms = vec ! [] ; let mut non_asms = vec ! [] ; for & (kind , span) in & this . items { match kind { ItemKind :: NakedAsm if has_naked_asm => { must_show_error = true ; multiple_asms . push (span) ; } ItemKind :: NakedAsm => has_naked_asm = true , ItemKind :: InlineAsm => { has_err = true ; tcx . dcx () . emit_err (NakedFunctionsMustNakedAsm { span }) ; } ItemKind :: NonAsm => { must_show_error = true ; non_asms . push (span) ; } ItemKind :: Err => has_err = true , } } if must_show_error || ! has_err { tcx . dcx () . emit_err (NakedFunctionsAsmBlock { span : tcx . def_span (def_id) , multiple_asms , non_asms , }) ; } } }
}
mkitem!{mkstruct!{struct CheckInlineAssembly { items : Vec < (ItemKind , Span) > , }}}
mkitem!{mkenum!{# [derive (Copy , Clone)] enum ItemKind { NakedAsm , InlineAsm , NonAsm , Err , }}}
mkitem!{mkimpl!{impl CheckInlineAssembly { fn check_expr < 'tcx > (& mut self , expr : & 'tcx hir :: Expr < 'tcx > , span : Span) { match expr . kind { ExprKind :: ConstBlock (..) | ExprKind :: Array (..) | ExprKind :: Call (..) | ExprKind :: MethodCall (..) | ExprKind :: Use (..) | ExprKind :: Tup (..) | ExprKind :: Binary (..) | ExprKind :: Unary (..) | ExprKind :: Lit (..) | ExprKind :: Cast (..) | ExprKind :: Type (..) | ExprKind :: UnsafeBinderCast (..) | ExprKind :: Loop (..) | ExprKind :: Match (..) | ExprKind :: If (..) | ExprKind :: Closure { .. } | ExprKind :: Assign (..) | ExprKind :: AssignOp (..) | ExprKind :: Field (..) | ExprKind :: Index (..) | ExprKind :: Path (..) | ExprKind :: AddrOf (..) | ExprKind :: Let (..) | ExprKind :: Break (..) | ExprKind :: Continue (..) | ExprKind :: Ret (..) | ExprKind :: OffsetOf (..) | ExprKind :: Become (..) | ExprKind :: Struct (..) | ExprKind :: Repeat (..) | ExprKind :: Yield (..) => { self . items . push ((ItemKind :: NonAsm , span)) ; } ExprKind :: InlineAsm (asm) => match asm . asm_macro { rustc_ast :: AsmMacro :: Asm => { self . items . push ((ItemKind :: InlineAsm , span)) ; } rustc_ast :: AsmMacro :: NakedAsm => { self . items . push ((ItemKind :: NakedAsm , span)) ; } rustc_ast :: AsmMacro :: GlobalAsm => { span_bug ! (span , "`global_asm!` is not allowed in this position") } } , ExprKind :: DropTemps (..) | ExprKind :: Block (..) => { hir :: intravisit :: walk_expr (self , expr) ; } ExprKind :: Err (_) => { self . items . push ((ItemKind :: Err , span)) ; } } } }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for CheckInlineAssembly { fn visit_stmt (& mut self , stmt : & 'tcx hir :: Stmt < 'tcx >) { match stmt . kind { StmtKind :: Item (..) => { } StmtKind :: Let (..) => { self . items . push ((ItemKind :: NonAsm , stmt . span)) ; } StmtKind :: Expr (expr) | StmtKind :: Semi (expr) => { self . check_expr (expr , stmt . span) ; } } } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { self . check_expr (expr , expr . span) ; } }}}