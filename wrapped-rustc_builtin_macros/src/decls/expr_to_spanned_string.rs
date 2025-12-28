macro_rules! deps {
    () => {
        ExprToSpannedString!();
        ExprToSpannedStringResult!();
    };
}

macro_rules! expr_to_spanned_string {
    () => {
        deps!();
        # [doc = " Extracts a string literal from the macro expanded version of `expr`,"] # [doc = " returning a diagnostic error of `err_msg` if `expr` is not a string literal."] # [doc = " The returned bool indicates whether an applicable suggestion has already been"] # [doc = " added to the diagnostic to avoid emitting multiple suggestions. `Err(Err(ErrorGuaranteed))`"] # [doc = " indicates that an ast error was encountered."] # [allow (rustc :: untranslatable_diagnostic)] pub (crate) fn expr_to_spanned_string < 'a > (cx : & 'a mut ExtCtxt < '_ > , expr : Box < ast :: Expr > , err_msg : & 'static str ,) -> ExpandResult < ExprToSpannedStringResult < 'a > , () > { if ! cx . force_mode && let ast :: ExprKind :: MacCall (m) = & expr . kind && cx . resolver . macro_accessible (cx . current_expansion . id , & m . path) . is_err () { return ExpandResult :: Retry (()) ; } let expr = cx . expander () . fully_expand_fragment (AstFragment :: Expr (expr)) . make_expr () ; ExpandResult :: Ready (Err (match expr . kind { ast :: ExprKind :: Lit (token_lit) => match ast :: LitKind :: from_token_lit (token_lit) { Ok (ast :: LitKind :: Str (s , style)) => { return ExpandResult :: Ready (Ok (ExprToSpannedString { symbol : s , style , span : expr . span , uncooked_symbol : (token_lit . kind , token_lit . symbol) , })) ; } Ok (ast :: LitKind :: ByteStr (..)) => { let mut err = cx . dcx () . struct_span_err (expr . span , err_msg) ; let span = expr . span . shrink_to_lo () ; err . span_suggestion (span . with_hi (span . lo () + BytePos (1)) , "consider removing the leading `b`" , "" , Applicability :: MaybeIncorrect ,) ; Ok ((err , true)) } Ok (ast :: LitKind :: Err (guar)) => Err (guar) , Err (err) => Err (report_lit_error (& cx . sess . psess , err , token_lit , expr . span)) , _ => Ok ((cx . dcx () . struct_span_err (expr . span , err_msg) , false)) , } , ast :: ExprKind :: Err (guar) => Err (guar) , ast :: ExprKind :: Dummy => { cx . dcx () . span_bug (expr . span , "tried to get a string literal from `ExprKind::Dummy`") } _ => Ok ((cx . dcx () . struct_span_err (expr . span , err_msg) , false)) , })) }
    };
}

expr_to_spanned_string!();