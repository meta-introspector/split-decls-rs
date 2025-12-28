macro_rules! deps {
    () => {
        DemangleContext!();
        ArgScopeStack!();
        Result!();
        Demangle!();
        DemangleWrite!();
        FunctionType!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for FunctionType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_inner (self) ; self . bare . demangle (ctx , scope) ? ; if ctx . pop_inner_if (self) { self . demangle_as_inner (ctx , scope) ? ; } if let Some (ref es) = self . exception_spec { ctx . ensure_space () ? ; es . demangle (ctx , scope) ? ; } Ok (()) } }
    };
}

impl_187!()