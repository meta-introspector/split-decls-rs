macro_rules! vals_for {
    () => {
        fn vals_for (o : & Arg) -> String { debug ! ("vals_for: o={}" , o . get_id ()) ; if let Some (vals) = utils :: possible_values (o) { format ! ("$(compgen -W \"{}\" -- \"${{cur}}\")" , vals . iter () . filter (| pv | ! pv . is_hide_set ()) . map (| n | n . get_name ()) . collect ::< Vec < _ >> () . join (" ")) } else if o . get_value_hint () == ValueHint :: DirPath { String :: from ("") } else if o . get_value_hint () == ValueHint :: Other { String :: from ("\"${cur}\"") } else { String :: from ("$(compgen -f \"${cur}\")") } }
    };
}

vals_for!();