macro_rules! deps {
    () => {
        Demangle!();
        Result!();
        DataMemberPrefix!();
        DemangleNodeType!();
        DemangleWrite!();
        ArgScopeStack!();
        DemangleContext!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for DataMemberPrefix where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; ctx . push_demangle_node (DemangleNodeType :: DataMemberPrefix) ; let ret = self . 0 . demangle (ctx , scope) ; ctx . pop_demangle_node () ; ret } }
    };
}

impl_291!();