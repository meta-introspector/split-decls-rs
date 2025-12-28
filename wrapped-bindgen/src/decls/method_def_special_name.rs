macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! method_def_special_name {
    () => {
        deps!();
        fn method_def_special_name (row : MethodDef) -> String { let name = row . name () ; if row . flags () . contains (MethodAttributes :: SpecialName) { if name . starts_with ("get") { name [4 ..] . to_string () } else if name . starts_with ("put") { format ! ("Set{}" , & name [4 ..]) } else if name . starts_with ("add") { name [4 ..] . to_string () } else if name . starts_with ("remove") { format ! ("Remove{}" , & name [7 ..]) } else { name . to_string () } } else { if let Some (attribute) = row . find_attribute ("OverloadAttribute") { for (_ , arg) in attribute . args () { if let Value :: Str (overload) = arg { if let Some (suffix) = overload . strip_prefix (name) { if suffix . parse :: < u32 > () . is_ok () { return name . to_string () ; } } return overload . to_string () ; } } } name . to_string () } }
    };
}

method_def_special_name!()