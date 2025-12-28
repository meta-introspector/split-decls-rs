macro_rules! call_intrinsic {
    () => {
        # [doc = " Constructs an expression that calls an intrinsic"] fn call_intrinsic (cx : & ExtCtxt < '_ > , span : Span , intrinsic : Symbol , args : ThinVec < Box < ast :: Expr > > ,) -> Box < ast :: Expr > { let span = cx . with_def_site_ctxt (span) ; let path = cx . std_path (& [sym :: intrinsics , intrinsic]) ; cx . expr_call_global (span , path , args) }
    };
}

call_intrinsic!()