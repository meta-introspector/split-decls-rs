macro_rules! DotAccessKind {
    () => {
        # [derive (Debug , Clone , Copy)] pub (crate) enum DotAccessKind { Field { # [doc = " True if the receiver is an integer and there is no ident in the original file after it yet"] # [doc = " like `0.$0`"] receiver_is_ambiguous_float_literal : bool , } , Method , }
    };
}

DotAccessKind!()