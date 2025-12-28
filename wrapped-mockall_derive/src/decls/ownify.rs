macro_rules! ownify {
    () => {
        # [doc = " Return the owned version of the input."] fn ownify (ty : & Type) -> Type { if let Type :: Reference (ref tr) = & ty { if tr . lifetime . as_ref () . is_some_and (| lt | lt . ident == "static") { ty . clone () } else { * tr . elem . clone () } } else { ty . clone () } }
    };
}

ownify!()