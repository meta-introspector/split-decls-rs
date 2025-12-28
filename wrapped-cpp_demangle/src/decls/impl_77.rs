macro_rules! deps {
    () => {
        ArgScopeStack!();
        GlobalCtorDtor!();
        Demangle!();
        Result!();
        DemangleContext!();
        DemangleWrite!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for GlobalCtorDtor where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; inner_barrier ! (ctx) ; let saved_show_params = ctx . show_params ; ctx . show_params = true ; let ret = match * self { GlobalCtorDtor :: Ctor (ref name) => { write ! (ctx , "global constructors keyed to ") ? ; name . demangle (ctx , scope) } GlobalCtorDtor :: Dtor (ref name) => { write ! (ctx , "global destructors keyed to ") ? ; name . demangle (ctx , scope) } } ; ctx . show_params = saved_show_params ; ret } }
    };
}

impl_77!()