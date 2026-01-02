mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: { GenericArg , MetaItem } ;}
mkuse!{use rustc_expand :: base :: { Annotatable , ExpandResult , ExtCtxt , MultiItemModifier } ;}
mkuse!{use rustc_span :: { Span , Symbol , sym } ;}
mkuse!{use thin_vec :: { ThinVec , thin_vec } ;}
mkitem!{macro path_local ($ x : ident) { generic :: ty :: Path :: new_local (sym ::$ x) }}
mkitem!{macro pathvec_std ($ ($ rest : ident) ::+) { { vec ! [$ (sym ::$ rest) ,+] } }}
mkitem!{macro path_std ($ ($ x : tt) *) { generic :: ty :: Path :: new (pathvec_std ! ($ ($ x) *)) }}
mkmod!{bounds, { 
                getname!(bounds);
                getsrc!(bounds);
                getpath!(bounds);
                get_deps!(bounds);
                get_crates!(bounds);
                mkinclude!(bounds);
                 
            }}
mkmod!{clone, { 
                getname!(clone);
                getsrc!(clone);
                getpath!(clone);
                get_deps!(clone);
                get_crates!(clone);
                mkinclude!(clone);
                 
            }}
mkmod!{coerce_pointee, { 
                getname!(coerce_pointee);
                getsrc!(coerce_pointee);
                getpath!(coerce_pointee);
                get_deps!(coerce_pointee);
                get_crates!(coerce_pointee);
                mkinclude!(coerce_pointee);
                 
            }}
mkmod!{debug, { 
                getname!(debug);
                getsrc!(debug);
                getpath!(debug);
                get_deps!(debug);
                get_crates!(debug);
                mkinclude!(debug);
                 
            }}
mkmod!{default, { 
                getname!(default);
                getsrc!(default);
                getpath!(default);
                get_deps!(default);
                get_crates!(default);
                mkinclude!(default);
                 
            }}
mkmod!{from, { 
                getname!(from);
                getsrc!(from);
                getpath!(from);
                get_deps!(from);
                get_crates!(from);
                mkinclude!(from);
                 
            }}
mkmod!{hash, { 
                getname!(hash);
                getsrc!(hash);
                getpath!(hash);
                get_deps!(hash);
                get_crates!(hash);
                mkinclude!(hash);
                 
            }}
mkmod!{eq, { 
                getname!(eq);
                getsrc!(eq);
                getpath!(eq);
                get_deps!(eq);
                get_crates!(eq);
                mkinclude!(eq);
                 
            }}
mkmod!{ord, { 
                getname!(ord);
                getsrc!(ord);
                getpath!(ord);
                get_deps!(ord);
                get_crates!(ord);
                mkinclude!(ord);
                 
            }}
mkmod!{partial_eq, { 
                getname!(partial_eq);
                getsrc!(partial_eq);
                getpath!(partial_eq);
                get_deps!(partial_eq);
                get_crates!(partial_eq);
                mkinclude!(partial_eq);
                 
            }}
mkmod!{partial_ord, { 
                getname!(partial_ord);
                getsrc!(partial_ord);
                getpath!(partial_ord);
                get_deps!(partial_ord);
                get_crates!(partial_ord);
                mkinclude!(partial_ord);
                 
            }}
mkmod!{generic, { 
                getname!(generic);
                getsrc!(generic);
                getpath!(generic);
                get_deps!(generic);
                get_crates!(generic);
                mkinclude!(generic);
                 
            }}
mkitem!{pub (crate) type BuiltinDeriveFn = fn (& ExtCtxt < '_ > , Span , & MetaItem , & Annotatable , & mut dyn FnMut (Annotatable) , bool) ;}
mkitem!{mkstruct!{pub (crate) struct BuiltinDerive (pub (crate) BuiltinDeriveFn) ;}}
mkitem!{mkimpl!{impl MultiItemModifier for BuiltinDerive { fn expand (& self , ecx : & mut ExtCtxt < '_ > , span : Span , meta_item : & MetaItem , item : Annotatable , is_derive_const : bool ,) -> ExpandResult < Vec < Annotatable > , Annotatable > { let span = ecx . with_def_site_ctxt (span) ; let mut items = Vec :: new () ; match item { Annotatable :: Stmt (stmt) => { if let ast :: StmtKind :: Item (item) = stmt . kind { (self . 0) (ecx , span , meta_item , & Annotatable :: Item (item) , & mut | a | { items . push (Annotatable :: Stmt (Box :: new (ast :: Stmt { id : ast :: DUMMY_NODE_ID , kind : ast :: StmtKind :: Item (a . expect_item ()) , span , }))) ; } , is_derive_const ,) ; } else { unreachable ! ("should have already errored on non-item statement") } } _ => { (self . 0) (ecx , span , meta_item , & item , & mut | a | items . push (a) , is_derive_const) ; } } ExpandResult :: Ready (items) } }}}

macro_rules! call_intrinsic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function call_intrinsic in module {}", module_path!());
    };
}

mkfn!{
    call_intrinsic_introspect!();
    # [doc = " Constructs an expression that calls an intrinsic"] fn call_intrinsic (cx : & ExtCtxt < '_ > , span : Span , intrinsic : Symbol , args : ThinVec < Box < ast :: Expr > > ,) -> Box < ast :: Expr > { let span = cx . with_def_site_ctxt (span) ; let path = cx . std_path (& [sym :: intrinsics , intrinsic]) ; cx . expr_call_global (span , path , args) }
}

macro_rules! call_unreachable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function call_unreachable in module {}", module_path!());
    };
}

mkfn!{
    call_unreachable_introspect!();
    # [doc = " Constructs an expression that calls the `unreachable` intrinsic."] fn call_unreachable (cx : & ExtCtxt < '_ > , span : Span) -> Box < ast :: Expr > { let span = cx . with_def_site_ctxt (span) ; let path = cx . std_path (& [sym :: intrinsics , sym :: unreachable]) ; let call = cx . expr_call_global (span , path , ThinVec :: new ()) ; cx . expr_block (Box :: new (ast :: Block { stmts : thin_vec ! [cx . stmt_expr (call)] , id : ast :: DUMMY_NODE_ID , rules : ast :: BlockCheckMode :: Unsafe (ast :: CompilerGenerated) , span , tokens : None , })) }
}

macro_rules! assert_ty_bounds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_ty_bounds in module {}", module_path!());
    };
}

mkfn!{
    assert_ty_bounds_introspect!();
    fn assert_ty_bounds (cx : & ExtCtxt < '_ > , stmts : & mut ThinVec < ast :: Stmt > , ty : Box < ast :: Ty > , span : Span , assert_path : & [Symbol] ,) { let span = cx . with_def_site_ctxt (span) ; let assert_path = cx . path_all (span , true , cx . std_path (assert_path) , vec ! [GenericArg :: Type (ty)]) ; stmts . push (cx . stmt_let_type_only (span , cx . ty_path (assert_path))) ; }
}