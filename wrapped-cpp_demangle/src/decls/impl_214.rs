macro_rules! deps {
    () => {
        VectorType!();
        Result!();
        DemangleContext!();
        ArgScopeStack!();
        DemangleWrite!();
        Demangle!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for VectorType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_inner (self) ; match * self { VectorType :: DimensionNumber (_ , ref ty) | VectorType :: DimensionExpression (_ , ref ty) => { ty . demangle (ctx , scope) ? ; } } if ctx . pop_inner_if (self) { self . demangle_as_inner (ctx , scope) ? ; } Ok (()) } }
    };
}

impl_214!()