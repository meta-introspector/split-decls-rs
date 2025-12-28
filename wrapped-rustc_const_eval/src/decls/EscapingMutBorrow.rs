macro_rules! EscapingMutBorrow {
    () => {
        # [derive (Debug)] # [doc = " This op is for `&mut` borrows in the trailing expression of a constant"] # [doc = " which uses the \"enclosing scopes rule\" to leak its locals into anonymous"] # [doc = " static or const items."] pub (crate) struct EscapingMutBorrow ;
    };
}

EscapingMutBorrow!();