macro_rules! deps {
    () => {
        OperatorName!();
        ArgScopeStack!();
        DemangleContext!();
        DemangleWrite!();
        Result!();
        Demangle!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for OperatorName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { OperatorName :: Simple (ref simple) => { match * simple { SimpleOperatorName :: New | SimpleOperatorName :: NewArray | SimpleOperatorName :: Delete | SimpleOperatorName :: DeleteArray => { ctx . ensure_space () ? ; } _ => { } } simple . demangle (ctx , scope) } OperatorName :: Cast (ref ty) | OperatorName :: Conversion (ref ty) => { inner_barrier ! (ctx) ; ctx . ensure_space () ? ; let scope = ty . get_template_args (ctx . subs) . map_or (scope , | args | scope . push (args)) ; ty . demangle (ctx , scope) ? ; Ok (()) } OperatorName :: Literal (ref name) => { name . demangle (ctx , scope) ? ; write ! (ctx , "::operator \"\"") ? ; Ok (()) } OperatorName :: VendorExtension (arity , ref name) => { name . demangle (ctx , scope) ? ; write ! (ctx , "::operator {}" , arity) ? ; Ok (()) } } } }
    };
}

impl_141!();