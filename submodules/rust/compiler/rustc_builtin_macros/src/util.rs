mkuse!{use rustc_ast :: tokenstream :: TokenStream ;}
mkuse!{use rustc_ast :: { self as ast , AttrStyle , Attribute , MetaItem , attr , token } ;}
mkuse!{use rustc_attr_parsing :: validate_attr ;}
mkuse!{use rustc_errors :: { Applicability , Diag , ErrorGuaranteed } ;}
mkuse!{use rustc_expand :: base :: { Annotatable , ExpandResult , ExtCtxt } ;}
mkuse!{use rustc_expand :: expand :: AstFragment ;}
mkuse!{use rustc_feature :: AttributeTemplate ;}
mkuse!{use rustc_lint_defs :: BuiltinLintDiag ;}
mkuse!{use rustc_lint_defs :: builtin :: DUPLICATE_MACRO_ATTRIBUTES ;}
mkuse!{use rustc_parse :: { exp , parser } ;}
mkuse!{use rustc_session :: errors :: report_lit_error ;}
mkuse!{use rustc_span :: { BytePos , Span , Symbol } ;}
mkuse!{use crate :: errors ;}

macro_rules! check_builtin_macro_attribute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_builtin_macro_attribute in module {}", module_path!());
    };
}

mkfn!{
    check_builtin_macro_attribute_introspect!();
    pub (crate) fn check_builtin_macro_attribute (ecx : & ExtCtxt < '_ > , meta_item : & MetaItem , name : Symbol) { let template = AttributeTemplate { word : true , .. Default :: default () } ; validate_attr :: check_builtin_meta_item (& ecx . sess . psess , meta_item , AttrStyle :: Outer , name , template , true ,) ; }
}

macro_rules! warn_on_duplicate_attribute_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function warn_on_duplicate_attribute in module {}", module_path!());
    };
}

mkfn!{
    warn_on_duplicate_attribute_introspect!();
    # [doc = " Emit a warning if the item is annotated with the given attribute. This is used to diagnose when"] # [doc = " an attribute may have been mistakenly duplicated."] pub (crate) fn warn_on_duplicate_attribute (ecx : & ExtCtxt < '_ > , item : & Annotatable , name : Symbol) { let attrs : Option < & [Attribute] > = match item { Annotatable :: Item (item) => Some (& item . attrs) , Annotatable :: AssocItem (item , _) => Some (& item . attrs) , Annotatable :: ForeignItem (item) => Some (& item . attrs) , Annotatable :: Expr (expr) => Some (& expr . attrs) , Annotatable :: Arm (arm) => Some (& arm . attrs) , Annotatable :: ExprField (field) => Some (& field . attrs) , Annotatable :: PatField (field) => Some (& field . attrs) , Annotatable :: GenericParam (param) => Some (& param . attrs) , Annotatable :: Param (param) => Some (& param . attrs) , Annotatable :: FieldDef (def) => Some (& def . attrs) , Annotatable :: Variant (variant) => Some (& variant . attrs) , _ => None , } ; if let Some (attrs) = attrs { if let Some (attr) = attr :: find_by_name (attrs , name) { ecx . psess () . buffer_lint (DUPLICATE_MACRO_ATTRIBUTES , attr . span , ecx . current_expansion . lint_node_id , BuiltinLintDiag :: DuplicateMacroAttribute ,) ; } } }
}
mkitem!{# [doc = " `Ok` represents successfully retrieving the string literal at the correct"] # [doc = " position, e.g., `println(\"abc\")`."] pub (crate) type ExprToSpannedStringResult < 'a > = Result < ExprToSpannedString , UnexpectedExprKind < 'a > > ;}
mkitem!{mkstruct!{pub (crate) struct ExprToSpannedString { pub symbol : Symbol , pub style : ast :: StrStyle , pub span : Span , # [doc = " The raw string literal, with no escaping or processing."] # [doc = ""] # [doc = " Generally only useful for lints that care about the raw bytes the user wrote."] pub uncooked_symbol : (ast :: token :: LitKind , Symbol) , }}}
mkitem!{# [doc = " - `Ok` is returned when the conversion to a string literal is unsuccessful,"] # [doc = " but another type of expression is obtained instead."] # [doc = " - `Err` is returned when the conversion process fails."] type UnexpectedExprKind < 'a > = Result < (Diag < 'a > , bool) , ErrorGuaranteed > ;}

macro_rules! expr_to_spanned_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_to_spanned_string in module {}", module_path!());
    };
}

mkfn!{
    expr_to_spanned_string_introspect!();
    # [doc = " Extracts a string literal from the macro expanded version of `expr`,"] # [doc = " returning a diagnostic error of `err_msg` if `expr` is not a string literal."] # [doc = " The returned bool indicates whether an applicable suggestion has already been"] # [doc = " added to the diagnostic to avoid emitting multiple suggestions. `Err(Err(ErrorGuaranteed))`"] # [doc = " indicates that an ast error was encountered."] # [allow (rustc :: untranslatable_diagnostic)] pub (crate) fn expr_to_spanned_string < 'a > (cx : & 'a mut ExtCtxt < '_ > , expr : Box < ast :: Expr > , err_msg : & 'static str ,) -> ExpandResult < ExprToSpannedStringResult < 'a > , () > { if ! cx . force_mode && let ast :: ExprKind :: MacCall (m) = & expr . kind && cx . resolver . macro_accessible (cx . current_expansion . id , & m . path) . is_err () { return ExpandResult :: Retry (()) ; } let expr = cx . expander () . fully_expand_fragment (AstFragment :: Expr (expr)) . make_expr () ; ExpandResult :: Ready (Err (match expr . kind { ast :: ExprKind :: Lit (token_lit) => match ast :: LitKind :: from_token_lit (token_lit) { Ok (ast :: LitKind :: Str (s , style)) => { return ExpandResult :: Ready (Ok (ExprToSpannedString { symbol : s , style , span : expr . span , uncooked_symbol : (token_lit . kind , token_lit . symbol) , })) ; } Ok (ast :: LitKind :: ByteStr (..)) => { let mut err = cx . dcx () . struct_span_err (expr . span , err_msg) ; let span = expr . span . shrink_to_lo () ; err . span_suggestion (span . with_hi (span . lo () + BytePos (1)) , "consider removing the leading `b`" , "" , Applicability :: MaybeIncorrect ,) ; Ok ((err , true)) } Ok (ast :: LitKind :: Err (guar)) => Err (guar) , Err (err) => Err (report_lit_error (& cx . sess . psess , err , token_lit , expr . span)) , _ => Ok ((cx . dcx () . struct_span_err (expr . span , err_msg) , false)) , } , ast :: ExprKind :: Err (guar) => Err (guar) , ast :: ExprKind :: Dummy => { cx . dcx () . span_bug (expr . span , "tried to get a string literal from `ExprKind::Dummy`") } _ => Ok ((cx . dcx () . struct_span_err (expr . span , err_msg) , false)) , })) }
}

macro_rules! expr_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_to_string in module {}", module_path!());
    };
}

mkfn!{
    expr_to_string_introspect!();
    # [doc = " Extracts a string literal from the macro expanded version of `expr`,"] # [doc = " emitting `err_msg` if `expr` is not a string literal. This does not stop"] # [doc = " compilation on error, merely emits a non-fatal error and returns `Err`."] pub (crate) fn expr_to_string (cx : & mut ExtCtxt < '_ > , expr : Box < ast :: Expr > , err_msg : & 'static str ,) -> ExpandResult < Result < (Symbol , ast :: StrStyle) , ErrorGuaranteed > , () > { expr_to_spanned_string (cx , expr , err_msg) . map (| res | { res . map_err (| err | match err { Ok ((err , _)) => err . emit () , Err (guar) => guar , }) . map (| ExprToSpannedString { symbol , style , .. } | (symbol , style)) }) }
}

macro_rules! check_zero_tts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_zero_tts in module {}", module_path!());
    };
}

mkfn!{
    check_zero_tts_introspect!();
    # [doc = " Non-fatally assert that `tts` is empty. Note that this function"] # [doc = " returns even when `tts` is non-empty, macros that *need* to stop"] # [doc = " compilation should call `cx.diagnostic().abort_if_errors()`"] # [doc = " (this should be done as rarely as possible)."] pub (crate) fn check_zero_tts (cx : & ExtCtxt < '_ > , span : Span , tts : TokenStream , name : & str) { if ! tts . is_empty () { cx . dcx () . emit_err (errors :: TakesNoArguments { span , name }) ; } }
}

macro_rules! parse_expr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_expr in module {}", module_path!());
    };
}

mkfn!{
    parse_expr_introspect!();
    # [doc = " Parse an expression. On error, emit it, advancing to `Eof`, and return `Err`."] pub (crate) fn parse_expr (p : & mut parser :: Parser < '_ >) -> Result < Box < ast :: Expr > , ErrorGuaranteed > { let guar = match p . parse_expr () { Ok (expr) => return Ok (expr) , Err (err) => err . emit () , } ; while p . token != token :: Eof { p . bump () ; } Err (guar) }
}

macro_rules! get_single_str_from_tts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_single_str_from_tts in module {}", module_path!());
    };
}

mkfn!{
    get_single_str_from_tts_introspect!();
    # [doc = " Interpreting `tts` as a comma-separated sequence of expressions,"] # [doc = " expect exactly one string literal, or emit an error and return `Err`."] pub (crate) fn get_single_str_from_tts (cx : & mut ExtCtxt < '_ > , span : Span , tts : TokenStream , name : & str ,) -> ExpandResult < Result < Symbol , ErrorGuaranteed > , () > { get_single_str_spanned_from_tts (cx , span , tts , name) . map (| res | res . map (| (s , _) | s)) }
}

macro_rules! get_single_str_spanned_from_tts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_single_str_spanned_from_tts in module {}", module_path!());
    };
}

mkfn!{
    get_single_str_spanned_from_tts_introspect!();
    pub (crate) fn get_single_str_spanned_from_tts (cx : & mut ExtCtxt < '_ > , span : Span , tts : TokenStream , name : & str ,) -> ExpandResult < Result < (Symbol , Span) , ErrorGuaranteed > , () > { let ExpandResult :: Ready (ret) = get_single_expr_from_tts (cx , span , tts , name) else { return ExpandResult :: Retry (()) ; } ; let ret = match ret { Ok (ret) => ret , Err (e) => return ExpandResult :: Ready (Err (e)) , } ; expr_to_spanned_string (cx , ret , "argument must be a string literal") . map (| res | { res . map_err (| err | match err { Ok ((err , _)) => err . emit () , Err (guar) => guar , }) . map (| ExprToSpannedString { symbol , span , .. } | (symbol , span)) }) }
}

macro_rules! get_single_expr_from_tts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_single_expr_from_tts in module {}", module_path!());
    };
}

mkfn!{
    get_single_expr_from_tts_introspect!();
    # [doc = " Interpreting `tts` as a comma-separated sequence of expressions,"] # [doc = " expect exactly one expression, or emit an error and return `Err`."] pub (crate) fn get_single_expr_from_tts (cx : & mut ExtCtxt < '_ > , span : Span , tts : TokenStream , name : & str ,) -> ExpandResult < Result < Box < ast :: Expr > , ErrorGuaranteed > , () > { let mut p = cx . new_parser_from_tts (tts) ; if p . token == token :: Eof { let guar = cx . dcx () . emit_err (errors :: OnlyOneArgument { span , name }) ; return ExpandResult :: Ready (Err (guar)) ; } let ret = match parse_expr (& mut p) { Ok (ret) => ret , Err (guar) => return ExpandResult :: Ready (Err (guar)) , } ; let _ = p . eat (exp ! (Comma)) ; if p . token != token :: Eof { cx . dcx () . emit_err (errors :: OnlyOneArgument { span , name }) ; } ExpandResult :: Ready (Ok (ret)) }
}

macro_rules! get_exprs_from_tts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_exprs_from_tts in module {}", module_path!());
    };
}

mkfn!{
    get_exprs_from_tts_introspect!();
    # [doc = " Extracts comma-separated expressions from `tts`."] # [doc = " On error, emit it, and return `Err`."] pub (crate) fn get_exprs_from_tts (cx : & mut ExtCtxt < '_ > , tts : TokenStream ,) -> ExpandResult < Result < Vec < Box < ast :: Expr > > , ErrorGuaranteed > , () > { let mut p = cx . new_parser_from_tts (tts) ; let mut es = Vec :: new () ; while p . token != token :: Eof { let expr = match parse_expr (& mut p) { Ok (expr) => expr , Err (guar) => return ExpandResult :: Ready (Err (guar)) , } ; if ! cx . force_mode && let ast :: ExprKind :: MacCall (m) = & expr . kind && cx . resolver . macro_accessible (cx . current_expansion . id , & m . path) . is_err () { return ExpandResult :: Retry (()) ; } let expr = cx . expander () . fully_expand_fragment (AstFragment :: Expr (expr)) . make_expr () ; es . push (expr) ; if p . eat (exp ! (Comma)) { continue ; } if p . token != token :: Eof { let guar = cx . dcx () . emit_err (errors :: ExpectedCommaInList { span : p . token . span }) ; return ExpandResult :: Ready (Err (guar)) ; } } ExpandResult :: Ready (Ok (es)) }
}