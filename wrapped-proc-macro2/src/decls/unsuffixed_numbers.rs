macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! unsuffixed_numbers {
    () => {
        deps!();
        macro_rules ! unsuffixed_numbers { ($ ($ name : ident => $ kind : ident ,) *) => ($ (pub (crate) fn $ name (n : $ kind) -> Literal { Literal :: _new (n . to_string ()) }) *) }
    };
}

unsuffixed_numbers!()