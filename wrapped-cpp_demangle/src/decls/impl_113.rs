macro_rules! deps {
    () => {
        UnqualifiedName!();
        Result!();
        DemangleContext!();
        DemangleWrite!();
        Demangle!();
        DemangleNodeType!();
        ArgScopeStack!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for UnqualifiedName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_demangle_node (DemangleNodeType :: UnqualifiedName) ; let ret = match * self { UnqualifiedName :: Operator (ref op_name , ref abi_tags) => { write ! (ctx , "operator") ? ; op_name . demangle (ctx , scope) ? ; abi_tags . demangle (ctx , scope) } UnqualifiedName :: CtorDtor (ref ctor_dtor , ref abi_tags) => { ctor_dtor . demangle (ctx , scope) ? ; abi_tags . demangle (ctx , scope) } UnqualifiedName :: Source (ref name , ref abi_tags) | UnqualifiedName :: LocalSourceName (ref name , _ , ref abi_tags) => { name . demangle (ctx , scope) ? ; abi_tags . demangle (ctx , scope) } UnqualifiedName :: UnnamedType (ref unnamed , ref abi_tags) => { unnamed . demangle (ctx , scope) ? ; abi_tags . demangle (ctx , scope) } UnqualifiedName :: ClosureType (ref closure , ref abi_tags) => { closure . demangle (ctx , scope) ? ; abi_tags . demangle (ctx , scope) } } ; ctx . pop_demangle_node () ; ret } }
    };
}

impl_113!();