macro_rules! possible_values {
    () => {
        # [doc = " Get the possible values for completion"] fn possible_values (a : & clap :: Arg) -> Option < Vec < clap :: builder :: PossibleValue > > { if ! a . get_num_args () . expect ("built") . takes_values () { None } else { a . get_value_parser () . possible_values () . map (| pvs | pvs . collect ()) } }
    };
}

possible_values!();