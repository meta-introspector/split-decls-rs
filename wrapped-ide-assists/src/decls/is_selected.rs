macro_rules! is_selected {
    () => {
        pub (crate) fn is_selected (it : & impl AstNode , selection : syntax :: TextRange , allow_empty : bool ,) -> bool { selection . intersect (it . syntax () . text_range ()) . is_some_and (| it | ! it . is_empty ()) || allow_empty && it . syntax () . text_range () . contains_range (selection) }
    };
}

is_selected!()