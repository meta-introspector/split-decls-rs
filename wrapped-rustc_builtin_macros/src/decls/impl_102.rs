macro_rules! deps {
    () => {
        PathKind!();
        Path!();
        Ty!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl Path { pub (crate) fn new (path : Vec < Symbol >) -> Path { Path :: new_ (path , Vec :: new () , PathKind :: Std) } pub (crate) fn new_local (path : Symbol) -> Path { Path :: new_ (vec ! [path] , Vec :: new () , PathKind :: Local) } pub (crate) fn new_ (path : Vec < Symbol > , params : Vec < Box < Ty > > , kind : PathKind) -> Path { Path { path , params , kind } } pub (crate) fn to_ty (& self , cx : & ExtCtxt < '_ > , span : Span , self_ty : Ident , self_generics : & Generics ,) -> Box < ast :: Ty > { cx . ty_path (self . to_path (cx , span , self_ty , self_generics)) } pub (crate) fn to_path (& self , cx : & ExtCtxt < '_ > , span : Span , self_ty : Ident , self_generics : & Generics ,) -> ast :: Path { let mut idents = self . path . iter () . map (| s | Ident :: new (* s , span)) . collect () ; let tys = self . params . iter () . map (| t | t . to_ty (cx , span , self_ty , self_generics)) ; let params = tys . map (GenericArg :: Type) . collect () ; match self . kind { PathKind :: Local => cx . path_all (span , false , idents , params) , PathKind :: Std => { let def_site = cx . with_def_site_ctxt (DUMMY_SP) ; idents . insert (0 , Ident :: new (kw :: DollarCrate , def_site)) ; cx . path_all (span , false , idents , params) } } } }
    };
}

impl_102!()