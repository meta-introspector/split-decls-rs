macro_rules! deps {
    () => {
        Demangle!();
        DemangleWrite!();
        ArgScopeStack!();
        Result!();
        Type!();
        DemangleContext!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for FunctionArgSlice where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; let mut saw_needs_paren = false ; let (needs_space , needs_paren) = ctx . inner . iter () . rev () . map (| inner | { if inner . downcast_to_pointer_to_member () . is_some () { (true , true) } else { match inner . downcast_to_type () { Some (& Type :: Qualified (..)) | Some (& Type :: Complex (_)) | Some (& Type :: Imaginary (_)) | Some (& Type :: PointerToMember (_)) => (true , true) , Some (& Type :: PointerTo (_)) | Some (& Type :: LvalueRef (_)) | Some (& Type :: RvalueRef (_)) => (false , true) , _ => (false , false) , } } }) . take_while (| & (_ , needs_paren) | { if saw_needs_paren { false } else { saw_needs_paren |= needs_paren ; true } }) . fold ((false , false) , | (space , paren) , (next_space , next_paren) | { (space || next_space , paren || next_paren) } ,) ; if needs_paren { let needs_space = needs_space || match ctx . last_char_written { Some ('(') | Some ('*') => false , _ => true , } ; if needs_space { ctx . ensure_space () ? ; } write ! (ctx , "(") ? ; } ctx . demangle_inner_prefixes (scope) ? ; if needs_paren { write ! (ctx , ")") ? ; } write ! (ctx , "(") ? ; if self . len () == 1 && self [0] . is_void () { write ! (ctx , ")") ? ; return Ok (()) ; } if ctx . is_explicit_obj_param { write ! (ctx , "this ") ? ; ctx . is_explicit_obj_param = false ; } let mut need_comma = false ; for arg in self . iter () { if need_comma { write ! (ctx , ", ") ? ; } arg . demangle (ctx , scope) ? ; need_comma = true ; } write ! (ctx , ")") ? ; ctx . demangle_inners (scope) } }
    };
}

impl_55!()