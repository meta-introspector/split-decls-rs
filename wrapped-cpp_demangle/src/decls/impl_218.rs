macro_rules! deps {
    () => {
        Demangle!();
        Result!();
        ArgScopeStack!();
        DemangleWrite!();
        PointerToMemberType!();
        DemangleContext!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for PointerToMemberType where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_inner (self) ; self . 1 . demangle (ctx , scope) ? ; if ctx . pop_inner_if (self) { self . demangle_as_inner (ctx , scope) ? ; } Ok (()) } }
    };
}

impl_218!();