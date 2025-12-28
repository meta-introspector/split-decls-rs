macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! generate_handler {
    () => {
        deps!();
        fn generate_handler (cx : & ExtCtxt < '_ > , handler : Ident , span : Span , sig_span : Span) -> Stmt { let usize = cx . path_ident (span , Ident :: new (sym :: usize , span)) ; let ty_usize = cx . ty_path (usize) ; let size = Ident :: new (sym :: size , span) ; let align = Ident :: new (sym :: align , span) ; let layout_new = cx . std_path (& [sym :: alloc , sym :: Layout , sym :: from_size_align_unchecked]) ; let layout_new = cx . expr_path (cx . path (span , layout_new)) ; let layout = cx . expr_call (span , layout_new , thin_vec ! [cx . expr_ident (span , size) , cx . expr_ident (span , align)] ,) ; let call = cx . expr_call_ident (sig_span , handler , thin_vec ! [layout]) ; let never = ast :: FnRetTy :: Ty (cx . ty (span , TyKind :: Never)) ; let params = thin_vec ! [cx . param (span , size , ty_usize . clone ()) , cx . param (span , align , ty_usize)] ; let decl = cx . fn_decl (params , never) ; let header = FnHeader { safety : Safety :: Unsafe (span) , .. FnHeader :: default () } ; let sig = FnSig { decl , header , span } ; let body = Some (cx . block_expr (call)) ; let kind = ItemKind :: Fn (Box :: new (Fn { defaultness : ast :: Defaultness :: Final , sig , ident : Ident :: from_str_and_span ("__rg_oom" , span) , generics : Generics :: default () , contract : None , body , define_opaque : None , })) ; let attrs = thin_vec ! [cx . attr_word (sym :: rustc_std_internal_symbol , span)] ; let item = cx . item (span , attrs , kind) ; cx . stmt_item (sig_span , item) }
    };
}

generate_handler!();