macro_rules! deps {
    () => {
        DemangleAsInner!();
        DemangleWrite!();
        DemangleContext!();
        Result!();
        ArgScopeStack!();
        VectorType!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for VectorType where W : 'subs + DemangleWrite , { fn demangle_as_inner < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle_as_inner ! (self , ctx , scope) ; match * self { VectorType :: DimensionNumber (n , _) => { write ! (ctx , " __vector({})" , n) ? ; } VectorType :: DimensionExpression (ref expr , _) => { write ! (ctx , " __vector(") ? ; expr . demangle (ctx , scope) ? ; write ! (ctx , ")") ? ; } } Ok (()) } }
    };
}

impl_215!()