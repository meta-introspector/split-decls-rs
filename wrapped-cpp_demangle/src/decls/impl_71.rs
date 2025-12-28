macro_rules! deps {
    () => {
        DemangleWrite!();
        DemangleAsInner!();
        Encoding!();
        DemangleContext!();
        Result!();
        ArgScopeStack!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for Encoding where W : 'subs + DemangleWrite , { fn demangle_as_inner < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { if let Encoding :: Function (ref name , ref fun_ty) = * self { let (scope , function_args) = if let Some (template_args) = name . get_template_args (ctx . subs) { let scope = scope . push (template_args) ; let function_args = FunctionArgListAndReturnType :: new (& fun_ty . 0) ; (scope , function_args as & dyn DemangleAsInner < W >) } else { let function_args = FunctionArgList :: new (& fun_ty . 0) ; (scope , function_args as & dyn DemangleAsInner < W >) } ; function_args . demangle_as_inner (ctx , scope) } else { unreachable ! ("we only push Encoding::Function onto the inner stack") ; } } }
    };
}

impl_71!()