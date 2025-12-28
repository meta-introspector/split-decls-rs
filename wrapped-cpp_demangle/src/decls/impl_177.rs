macro_rules! deps {
    () => {
        DemangleContext!();
        Result!();
        DemangleWrite!();
        Demangle!();
        BuiltinType!();
        ArgScopeStack!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for BuiltinType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { BuiltinType :: Standard (ref ty) => ty . demangle (ctx , scope) , BuiltinType :: Parametric (ref ty) => ty . demangle (ctx , scope) , BuiltinType :: Extension (ref name) => name . demangle (ctx , scope) , } } }
    };
}

impl_177!();