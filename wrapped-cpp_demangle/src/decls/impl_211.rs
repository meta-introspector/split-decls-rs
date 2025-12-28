macro_rules! deps {
    () => {
        DemangleWrite!();
        DemangleContext!();
        Type!();
        ArrayType!();
        Result!();
        ArgScopeStack!();
        DemangleAsInner!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsInner < 'subs , W > for ArrayType where W : 'subs + DemangleWrite , { fn demangle_as_inner < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle_as_inner ! (self , ctx , scope) ; let mut needs_space = true ; while let Some (inner) = ctx . pop_inner () { let inner_is_array = match inner . downcast_to_type () { Some (& Type :: Qualified (_ , ref ty)) => ctx . subs . get_type (ty) . map_or (false , | ty | { DemangleAsInner :: < W > :: downcast_to_array_type (ty) . is_some () }) , _ => { if inner . downcast_to_array_type () . is_some () { needs_space = false ; true } else { false } } } ; if inner_is_array { inner . demangle_as_inner (ctx , scope) ? ; } else { ctx . ensure_space () ? ; if inner . is_qualified () { inner . demangle_as_inner (ctx , scope) ? ; ctx . ensure_space () ? ; write ! (ctx , "(") ? ; } else { write ! (ctx , "(") ? ; inner . demangle_as_inner (ctx , scope) ? ; } ctx . demangle_inners (scope) ? ; write ! (ctx , ")") ? ; } } if needs_space { ctx . ensure_space () ? ; } match * self { ArrayType :: DimensionNumber (n , _) => { write ! (ctx , "[{}]" , n) ? ; } ArrayType :: DimensionExpression (ref expr , _) => { write ! (ctx , "[") ? ; expr . demangle (ctx , scope) ? ; write ! (ctx , "]") ? ; } ArrayType :: NoDimension (_) => { write ! (ctx , "[]") ? ; } } Ok (()) } fn downcast_to_array_type (& self) -> Option < & ArrayType > { Some (self) } }
    };
}

impl_211!();