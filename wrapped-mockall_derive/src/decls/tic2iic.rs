macro_rules! tic2iic {
    () => {
        # [doc = " Converts a TraitItemConst into an ImplItemConst"] fn tic2iic (tic : TraitItemConst , vis : & syn :: Visibility) -> ImplItemConst { let span = tic . span () ; let (eq_token , expr) = tic . default . unwrap_or_else (| | { compile_error (span , "Mocked associated consts must have a default implementation") ; (< Token ! [=] > :: default () , Expr :: Verbatim (TokenStream :: new ())) }) ; ImplItemConst { attrs : tic . attrs , vis : vis . clone () , defaultness : None , const_token : tic . const_token , generics : tic . generics , ident : tic . ident , colon_token : tic . colon_token , ty : tic . ty , eq_token , expr , semi_token : tic . semi_token } }
    };
}

tic2iic!();