macro_rules! deps {
    () => {
        Result!();
        ArgScopeStack!();
        Name!();
        DemangleContext!();
        DemangleWrite!();
        Demangle!();
        BaseUnresolvedName!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for BaseUnresolvedName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { BaseUnresolvedName :: Name (ref name) => name . demangle (ctx , scope) , BaseUnresolvedName :: Destructor (ref dtor) => dtor . demangle (ctx , scope) , BaseUnresolvedName :: Operator (ref op , ref args) => { op . demangle (ctx , scope) ? ; if let Some (ref args) = * args { args . demangle (ctx , scope) ? ; } Ok (()) } } } }
    };
}

impl_261!();