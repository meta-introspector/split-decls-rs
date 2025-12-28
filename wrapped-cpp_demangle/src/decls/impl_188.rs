macro_rules! deps {
    () => {
        Result!();
        ArgScopeStack!();
        DemangleContext!();
        DemangleAsInner!();
        FunctionType!();
        DemangleWrite!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for FunctionType where W : 'subs + DemangleWrite , { fn demangle_as_inner < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle_as_inner ! (self , ctx , scope) ; if ! self . cv_qualifiers . is_empty () { self . cv_qualifiers . demangle (ctx , scope) ? ; } if let Some (ref rq) = self . ref_qualifier { ctx . ensure_space () ? ; rq . demangle (ctx , scope) ? ; } Ok (()) } fn downcast_to_function_type (& self) -> Option < & FunctionType > { Some (self) } }
    };
}

impl_188!()