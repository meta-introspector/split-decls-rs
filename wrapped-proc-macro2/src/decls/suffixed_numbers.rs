macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! suffixed_numbers {
    () => {
        deps!();
        macro_rules ! suffixed_numbers { ($ ($ name : ident => $ kind : ident ,) *) => ($ (pub (crate) fn $ name (n : $ kind) -> Literal { Literal :: _new (format ! (concat ! ("{}" , stringify ! ($ kind)) , n)) }) *) }
    };
}

suffixed_numbers!();