macro_rules! deps {
    () => {
        GrastTriple!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl GrastTriple { pub fn to_turtle (& self) -> String { format ! ("{} {} {} ." , self . subject , self . predicate , self . object) } pub fn from_turtle (line : & str) -> Option < Self > { let parts : Vec < & str > = line . trim_end_matches ('.') . split_whitespace () . collect () ; if parts . len () >= 3 { Some (GrastTriple { subject : parts [0] . to_string () , predicate : parts [1] . to_string () , object : parts [2 ..] . join (" ") , }) } else { None } } }
    };
}

impl_3!();