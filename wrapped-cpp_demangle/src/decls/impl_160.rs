macro_rules! deps {
    () => {
        TemplateParam!();
        DemangleWrite!();
        Result!();
        Type!();
        Demangle!();
        DemangleContext!();
        Decltype!();
        ArgScopeStack!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for Type where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; match * self { Type :: Function (ref func_ty) => func_ty . demangle (ctx , scope) , Type :: ClassEnum (ref cls_enum_ty) => cls_enum_ty . demangle (ctx , scope) , Type :: Array (ref array_ty) => array_ty . demangle (ctx , scope) , Type :: Vector (ref vector_ty) => vector_ty . demangle (ctx , scope) , Type :: PointerToMember (ref ptm) => ptm . demangle (ctx , scope) , Type :: TemplateParam (ref param) => param . demangle (ctx , scope) , Type :: TemplateTemplate (ref tt_param , ref args) => { tt_param . demangle (ctx , scope) ? ; args . demangle (ctx , scope) } Type :: Decltype (ref dt) => dt . demangle (ctx , scope) , Type :: Qualified (_ , ref ty) => { ctx . push_inner (self) ; ty . demangle (ctx , scope) ? ; if ctx . pop_inner_if (self) { self . demangle_as_inner (ctx , scope) ? ; } Ok (()) } Type :: PointerTo (ref ty) | Type :: LvalueRef (ref ty) | Type :: RvalueRef (ref ty) => { ctx . push_inner (self) ; ty . demangle (ctx , scope) ? ; if ctx . pop_inner_if (self) { self . demangle_as_inner (ctx , scope) ? ; } Ok (()) } Type :: Complex (ref ty) => { ty . demangle (ctx , scope) ? ; write ! (ctx , " complex") ? ; Ok (()) } Type :: Imaginary (ref ty) => { ty . demangle (ctx , scope) ? ; write ! (ctx , " imaginary") ? ; Ok (()) } Type :: VendorExtension (ref name , ref template_args , ref ty) => { ty . demangle (ctx , scope) ? ; write ! (ctx , " ") ? ; name . demangle (ctx , scope) ? ; if let Some (ref args) = * template_args { args . demangle (ctx , scope) ? ; } Ok (()) } Type :: PackExpansion (ref ty) => { ty . demangle (ctx , scope) ? ; if ! ctx . is_template_argument_pack { write ! (ctx , "...") ? ; } Ok (()) } } } }
    };
}

impl_160!();