macro_rules! deps {
    () => {
        Name!();
        Demangle!();
        DestructorName!();
        DemangleWrite!();
        ArgScopeStack!();
        Result!();
        DemangleContext!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for DestructorName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; write ! (ctx , "~") ? ; match * self { DestructorName :: Unresolved (ref ty) => ty . demangle (ctx , scope) , DestructorName :: Name (ref name) => name . demangle (ctx , scope) , } } }
    };
}

impl_264!();