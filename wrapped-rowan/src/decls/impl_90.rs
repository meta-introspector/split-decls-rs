macro_rules! deps {
    () => {
        SyntaxText!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl PartialEq < str > for SyntaxText { fn eq (& self , mut rhs : & str) -> bool { self . try_for_each_chunk (| chunk | { if ! rhs . starts_with (chunk) { return Err (()) ; } rhs = & rhs [chunk . len () ..] ; Ok (()) }) . is_ok () && rhs . is_empty () } }
    };
}

impl_90!()