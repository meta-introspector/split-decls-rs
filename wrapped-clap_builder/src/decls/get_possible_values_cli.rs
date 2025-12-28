macro_rules! deps {
    () => {
        PossibleValue!();
        Arg!();
    };
}

macro_rules! get_possible_values_cli {
    () => {
        deps!();
        pub (crate) fn get_possible_values_cli (a : & Arg) -> Vec < PossibleValue > { if ! a . is_takes_value_set () { vec ! [] } else { a . get_value_parser () . possible_values () . map (| pvs | pvs . collect ()) . unwrap_or_default () } }
    };
}

get_possible_values_cli!();