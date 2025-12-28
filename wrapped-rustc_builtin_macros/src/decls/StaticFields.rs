macro_rules! deps {
    () => {
        IsTuple!();
    };
}

macro_rules! StaticFields {
    () => {
        deps!();
        # [doc = " Fields for a static method"] pub (crate) enum StaticFields { # [doc = " Tuple and unit structs/enum variants like this."] Unnamed (Vec < Span > , IsTuple) , # [doc = " Normal structs/struct variants."] Named (Vec < (Ident , Span , Option < AnonConst >) >) , }
    };
}

StaticFields!()