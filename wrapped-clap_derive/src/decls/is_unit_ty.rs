macro_rules! is_unit_ty {
    () => {
        fn is_unit_ty (ty : & Type) -> bool { if let Type :: Tuple (tuple) = ty { tuple . elems . is_empty () } else { false } }
    };
}

is_unit_ty!()