macro_rules! deps {
    () => {
        Ty!();
        Path!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl Ty { pub (crate) fn to_ty (& self , cx : & ExtCtxt < '_ > , span : Span , self_ty : Ident , self_generics : & Generics ,) -> Box < ast :: Ty > { match self { Ref (ty , mutbl) => { let raw_ty = ty . to_ty (cx , span , self_ty , self_generics) ; cx . ty_ref (span , raw_ty , None , * mutbl) } Path (p) => p . to_ty (cx , span , self_ty , self_generics) , Self_ => cx . ty_path (self . to_path (cx , span , self_ty , self_generics)) , Unit => { let ty = ast :: TyKind :: Tup (ThinVec :: new ()) ; cx . ty (span , ty) } AstTy (ty) => ty . clone () , } } pub (crate) fn to_path (& self , cx : & ExtCtxt < '_ > , span : Span , self_ty : Ident , generics : & Generics ,) -> ast :: Path { match self { Self_ => { let params : Vec < _ > = generics . params . iter () . map (| param | match param . kind { GenericParamKind :: Lifetime { .. } => { GenericArg :: Lifetime (ast :: Lifetime { id : param . id , ident : param . ident }) } GenericParamKind :: Type { .. } => { GenericArg :: Type (cx . ty_ident (span , param . ident)) } GenericParamKind :: Const { .. } => { GenericArg :: Const (cx . const_ident (span , param . ident)) } }) . collect () ; cx . path_all (span , false , vec ! [self_ty] , params) } Path (p) => p . to_path (cx , span , self_ty , generics) , AstTy (ty) => match & ty . kind { TyKind :: Path (_ , path) => path . clone () , _ => cx . dcx () . span_bug (span , "non-path in a path in generic `derive`") , } , Ref (..) => cx . dcx () . span_bug (span , "ref in a path in generic `derive`") , Unit => cx . dcx () . span_bug (span , "unit in a path in generic `derive`") , } } }
    };
}

impl_105!();