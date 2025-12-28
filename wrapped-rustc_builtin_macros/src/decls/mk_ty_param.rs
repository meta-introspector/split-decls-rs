macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! mk_ty_param {
    () => {
        deps!();
        fn mk_ty_param (cx : & ExtCtxt < '_ > , span : Span , name : Symbol , bounds : & [Path] , self_ident : Ident , self_generics : & Generics ,) -> ast :: GenericParam { let bounds = bounds . iter () . map (| b | { let path = b . to_path (cx , span , self_ident , self_generics) ; cx . trait_bound (path , false) }) . collect () ; cx . typaram (span , Ident :: new (name , span) , bounds , None) }
    };
}

mk_ty_param!();