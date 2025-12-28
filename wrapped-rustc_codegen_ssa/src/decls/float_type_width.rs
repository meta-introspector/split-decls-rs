macro_rules! float_type_width {
    () => {
        fn float_type_width (ty : Ty < '_ >) -> Option < u64 > { match ty . kind () { ty :: Float (t) => Some (t . bit_width ()) , _ => None , } }
    };
}

float_type_width!()