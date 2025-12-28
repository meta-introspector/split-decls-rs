macro_rules! get_explicit_self {
    () => {
        pub (crate) fn get_explicit_self (cx : & ExtCtxt < '_ > , span : Span) -> (Box < Expr > , ast :: ExplicitSelf) { let self_path = cx . expr_self (span) ; let self_ty = respan (span , SelfKind :: Region (None , ast :: Mutability :: Not)) ; (self_path , self_ty) }
    };
}

get_explicit_self!()