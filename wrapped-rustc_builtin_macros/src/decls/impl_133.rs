macro_rules! deps {
    () => {
        BuiltinDerive!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl MultiItemModifier for BuiltinDerive { fn expand (& self , ecx : & mut ExtCtxt < '_ > , span : Span , meta_item : & MetaItem , item : Annotatable , is_derive_const : bool ,) -> ExpandResult < Vec < Annotatable > , Annotatable > { let span = ecx . with_def_site_ctxt (span) ; let mut items = Vec :: new () ; match item { Annotatable :: Stmt (stmt) => { if let ast :: StmtKind :: Item (item) = stmt . kind { (self . 0) (ecx , span , meta_item , & Annotatable :: Item (item) , & mut | a | { items . push (Annotatable :: Stmt (Box :: new (ast :: Stmt { id : ast :: DUMMY_NODE_ID , kind : ast :: StmtKind :: Item (a . expect_item ()) , span , }))) ; } , is_derive_const ,) ; } else { unreachable ! ("should have already errored on non-item statement") } } _ => { (self . 0) (ecx , span , meta_item , & item , & mut | a | items . push (a) , is_derive_const) ; } } ExpandResult :: Ready (items) } }
    };
}

impl_133!()