macro_rules! deps {
    () => {
        ArgScopeStack!();
        DemangleContext!();
        DemangleNodeType!();
        Demangle!();
        SpecialName!();
        DemangleWrite!();
        Result!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for SpecialName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { SpecialName :: VirtualTable (ref ty) => { write ! (ctx , "{{vtable(") ? ; ctx . push_demangle_node (DemangleNodeType :: VirtualTable) ; ty . demangle (ctx , scope) ? ; ctx . pop_demangle_node () ; write ! (ctx , ")}}") ? ; Ok (()) } SpecialName :: Vtt (ref ty) => { write ! (ctx , "{{vtt(") ? ; ty . demangle (ctx , scope) ? ; write ! (ctx , ")}}") ? ; Ok (()) } SpecialName :: Typeinfo (ref ty) => { write ! (ctx , "typeinfo for ") ? ; ty . demangle (ctx , scope) } SpecialName :: TypeinfoName (ref ty) => { write ! (ctx , "typeinfo name for ") ? ; ty . demangle (ctx , scope) } SpecialName :: VirtualOverrideThunk (ref offset , ref encoding) => { write ! (ctx , "{{virtual override thunk(") ? ; offset . demangle (ctx , scope) ? ; write ! (ctx , ", ") ? ; encoding . demangle (ctx , scope) ? ; write ! (ctx , ")}}") ? ; Ok (()) } SpecialName :: VirtualOverrideThunkCovariant (ref this_offset , ref result_offset , ref encoding ,) => { write ! (ctx , "{{virtual override thunk(") ? ; this_offset . demangle (ctx , scope) ? ; write ! (ctx , ", ") ? ; result_offset . demangle (ctx , scope) ? ; write ! (ctx , ", ") ? ; encoding . demangle (ctx , scope) ? ; write ! (ctx , ")}}") ? ; Ok (()) } SpecialName :: Guard (ref name) => { write ! (ctx , "guard variable for ") ? ; name . demangle (ctx , scope) } SpecialName :: GuardTemporary (ref name , n) => { write ! (ctx , "reference temporary #{} for " , n) ? ; name . demangle (ctx , scope) } SpecialName :: ConstructionVtable (ref ty1 , _ , ref ty2) => { write ! (ctx , "construction vtable for ") ? ; ty1 . demangle (ctx , scope) ? ; write ! (ctx , "-in-") ? ; ty2 . demangle (ctx , scope) } SpecialName :: TypeinfoFunction (ref ty) => { write ! (ctx , "typeinfo fn for ") ? ; ty . demangle (ctx , scope) } SpecialName :: TlsInit (ref name) => { write ! (ctx , "TLS init function for ") ? ; name . demangle (ctx , scope) } SpecialName :: TlsWrapper (ref name) => { write ! (ctx , "TLS wrapper function for ") ? ; name . demangle (ctx , scope) } SpecialName :: TransactionClone (ref encoding) => { write ! (ctx , "transaction clone for ") ? ; encoding . demangle (ctx , scope) } SpecialName :: NonTransactionClone (ref encoding) => { write ! (ctx , "non-transaction clone for ") ? ; encoding . demangle (ctx , scope) } SpecialName :: JavaResource (ref names) => { write ! (ctx , "java resource ") ? ; for name in names { name . demangle (ctx , scope) ? ; } Ok (()) } } } }
    };
}

impl_300!();