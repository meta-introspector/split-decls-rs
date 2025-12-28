macro_rules! supersuperfy_generics {
    () => {
        fn supersuperfy_generics (generics : & mut Generics , levels : usize) { for param in generics . params . iter_mut () { if let GenericParam :: Type (tp) = param { supersuperfy_bounds (& mut tp . bounds , levels) ; if let Some (ty) = tp . default . as_mut () { * ty = supersuperfy (ty , levels) ; } } } if let Some (wc) = generics . where_clause . as_mut () { for wp in wc . predicates . iter_mut () { if let WherePredicate :: Type (pt) = wp { pt . bounded_ty = supersuperfy (& pt . bounded_ty , levels) ; supersuperfy_bounds (& mut pt . bounds , levels) ; } } } }
    };
}

supersuperfy_generics!();