macro_rules! deps {
    () => {
        ConcatBytesArray!();
    };
}

macro_rules! handle_array_element {
    () => {
        deps!();
        # [doc = " Returns `expr` as a *single* byte literal if applicable."] # [doc = ""] # [doc = " Otherwise, returns `None`, and either pushes the `expr`'s span to `missing_literals` or"] # [doc = " updates `guar` accordingly."] fn handle_array_element (cx : & ExtCtxt < '_ > , guar : & mut Option < ErrorGuaranteed > , missing_literals : & mut Vec < rustc_span :: Span > , expr : & Box < rustc_ast :: Expr > ,) -> Option < u8 > { let dcx = cx . dcx () ; match expr . kind { ExprKind :: Lit (token_lit) => { match LitKind :: from_token_lit (token_lit) { Ok (LitKind :: Int (val , LitIntType :: Unsuffixed | LitIntType :: Unsigned (UintTy :: U8) ,)) if let Ok (val) = u8 :: try_from (val . get ()) => { return Some (val) ; } Ok (LitKind :: Byte (val)) => return Some (val) , Ok (LitKind :: ByteStr (..)) => { guar . get_or_insert_with (| | { dcx . emit_err (errors :: ConcatBytesArray { span : expr . span , bytestr : true }) }) ; } _ => { guar . get_or_insert_with (| | invalid_type_err (cx , token_lit , expr . span , true)) ; } } ; } ExprKind :: Array (_) | ExprKind :: Repeat (_ , _) => { guar . get_or_insert_with (| | { dcx . emit_err (errors :: ConcatBytesArray { span : expr . span , bytestr : false }) }) ; } ExprKind :: IncludedBytes (..) => { guar . get_or_insert_with (| | { dcx . emit_err (errors :: ConcatBytesArray { span : expr . span , bytestr : false }) }) ; } _ => missing_literals . push (expr . span) , } None }
    };
}

handle_array_element!();