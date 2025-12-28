macro_rules! deps {
    () => {
        ArgScopeStack!();
        Error!();
        CtorDtorName!();
        DemangleWrite!();
        Demangle!();
        Result!();
        DemangleContext!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for CtorDtorName where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; let leaf = scope . leaf_name () . map_err (| e | { log ! ("Error getting leaf name: {}" , e) ; fmt :: Error }) ? ; match * self { CtorDtorName :: CompleteConstructor (ref inheriting) | CtorDtorName :: BaseConstructor (ref inheriting) | CtorDtorName :: CompleteAllocatingConstructor (ref inheriting) | CtorDtorName :: MaybeInChargeConstructor (ref inheriting) => match inheriting { Some (ty) => ty . get_leaf_name (ctx . subs) . ok_or_else (| | { log ! ("Error getting leaf name: {:?}" , ty) ; fmt :: Error }) ? . demangle_as_leaf (ctx) , None => leaf . demangle_as_leaf (ctx) , } , CtorDtorName :: DeletingDestructor | CtorDtorName :: CompleteDestructor | CtorDtorName :: BaseDestructor | CtorDtorName :: MaybeInChargeDestructor => { write ! (ctx , "~") ? ; leaf . demangle_as_leaf (ctx) } } } }
    };
}

impl_153!()