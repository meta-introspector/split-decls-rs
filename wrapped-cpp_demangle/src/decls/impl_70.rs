macro_rules! deps {
    () => {
        SourceName!();
        Encoding!();
        DemangleContext!();
        Demangle!();
        DemangleWrite!();
        LeafName!();
        Result!();
        ArgScopeStack!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for Encoding where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; inner_barrier ! (ctx) ; match * self { Encoding :: Function (ref name , ref fun_ty) => { debug_assert ! (! fun_ty . 0 . is_empty ()) ; let scope = if let Some (leaf) = name . get_leaf_name (ctx . subs) { match leaf { LeafName :: SourceName (leaf) => scope . push (leaf) , LeafName :: WellKnownComponent (leaf) => scope . push (leaf) , LeafName :: Closure (leaf) => scope . push (leaf) , LeafName :: UnnamedType (leaf) => scope . push (leaf) , } } else { scope } ; let scope = if let Some (template_args) = name . get_template_args (ctx . subs) { let scope = scope . push (template_args) ; if ctx . show_return_type && ! name . is_ctor_dtor_conversion (ctx . subs) { fun_ty . 0 [0] . demangle (ctx , scope) ? ; write ! (ctx , " ") ? ; } scope } else { scope } ; if ctx . show_params { ctx . push_inner (self) ; name . demangle (ctx , scope) ? ; if ctx . pop_inner_if (self) { self . demangle_as_inner (ctx , scope) ? ; } } else { name . demangle (ctx , scope) ? ; } Ok (()) } Encoding :: Data (ref name) => name . demangle (ctx , scope) , Encoding :: Special (ref name) => name . demangle (ctx , scope) , } } }
    };
}

impl_70!()