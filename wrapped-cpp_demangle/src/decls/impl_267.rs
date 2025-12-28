macro_rules! deps {
    () => {
        DemangleContext!();
        Result!();
        Error!();
        BuiltinType!();
        Demangle!();
        ExprPrimary!();
        DemangleWrite!();
        ArgScopeStack!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for ExprPrimary where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; fn write_literal < W > (ctx : & mut DemangleContext < W > , start : usize , end : usize) -> fmt :: Result where W : DemangleWrite , { debug_assert ! (start <= end) ; let start = if start < end && ctx . input [start] == b'n' { write ! (ctx , "-") ? ; start + 1 } else { start } ; let s = str :: from_utf8 (& ctx . input [start .. end]) . map_err (| e | { log ! ("Error writing literal: {}" , e) ; fmt :: Error }) ? ; ctx . write_str (s) } match * self { ExprPrimary :: External (ref name) => { let saved_show_params = ctx . show_params ; ctx . show_params = true ; let ret = name . demangle (ctx , scope) ; ctx . show_params = saved_show_params ; ret } ExprPrimary :: Literal (TypeHandle :: Builtin (BuiltinType :: Standard (StandardBuiltinType :: Bool)) , start , end ,) => match & ctx . input [start .. end] { b"0" => write ! (ctx , "false") , b"1" => write ! (ctx , "true") , _ => { write ! (ctx , "(bool)") ? ; write_literal (ctx , start , end) } } , ExprPrimary :: Literal (TypeHandle :: Builtin (BuiltinType :: Standard (StandardBuiltinType :: Nullptr)) , _ , _ ,) => write ! (ctx , "nullptr") , ExprPrimary :: Literal (ref ty @ TypeHandle :: Builtin (BuiltinType :: Standard (StandardBuiltinType :: Double)) , start , end ,) | ExprPrimary :: Literal (ref ty @ TypeHandle :: Builtin (BuiltinType :: Standard (StandardBuiltinType :: Float)) , start , end ,) => { if ctx . show_expression_literal_types { write ! (ctx , "(") ? ; ty . demangle (ctx , scope) ? ; write ! (ctx , ")") ? ; } let start = if start < end && ctx . input [start] == b'n' { write ! (ctx , "-[") ? ; start + 1 } else { write ! (ctx , "[") ? ; start } ; let s = str :: from_utf8 (& ctx . input [start .. end]) . map_err (| e | { log ! ("Error writing literal: {}" , e) ; fmt :: Error }) ? ; ctx . write_str (s) ? ; write ! (ctx , "]") } ExprPrimary :: Literal (TypeHandle :: Builtin (BuiltinType :: Standard (StandardBuiltinType :: Int)) , start , end ,) => write_literal (ctx , start , end) , ExprPrimary :: Literal (ref ty , start , end) => { if ctx . show_expression_literal_types { write ! (ctx , "(") ? ; ty . demangle (ctx , scope) ? ; write ! (ctx , ")") ? ; } write_literal (ctx , start , end) } } } }
    };
}

impl_267!()