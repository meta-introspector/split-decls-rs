macro_rules! deps {
    () => {
        Type!();
        TemplateTemplateParam!();
        Substitutable!();
        Demangle!();
        UnresolvedType!();
        ArgScopeStack!();
        Prefix!();
        DemangleWrite!();
        Result!();
        UnscopedTemplateName!();
        DemangleContext!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < 'subs , W > ast :: Demangle < 'subs , W > for Substitutable where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut ast :: DemangleContext < 'subs , W > , scope : Option < ast :: ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { match * self { Substitutable :: UnscopedTemplateName (ref name) => name . demangle (ctx , scope) , Substitutable :: Type (ref ty) => ty . demangle (ctx , scope) , Substitutable :: TemplateTemplateParam (ref ttp) => ttp . demangle (ctx , scope) , Substitutable :: UnresolvedType (ref ty) => ty . demangle (ctx , scope) , Substitutable :: Prefix (ref prefix) => prefix . demangle (ctx , scope) , } } }
    };
}

impl_332!();