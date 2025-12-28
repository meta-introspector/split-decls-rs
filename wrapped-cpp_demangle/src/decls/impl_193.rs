macro_rules! deps {
    () => {
        DemangleWrite!();
        ArgScopeStack!();
        BareFunctionType!();
        Demangle!();
        DemangleContext!();
        Result!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for BareFunctionType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_inner (self) ; self . ret () . demangle (ctx , scope) ? ; if ctx . pop_inner_if (self) { ctx . ensure_space () ? ; self . demangle_as_inner (ctx , scope) ? ; } Ok (()) } }
    };
}

impl_193!();