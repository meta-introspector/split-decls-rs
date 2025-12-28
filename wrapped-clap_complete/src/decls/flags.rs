macro_rules! flags {
    () => {
        # [doc = " Gets all the flags of a [`clap::Command`]."] # [doc = " Includes `help` and `version` depending on the [`clap::Command`] settings."] pub fn flags (p : & Command) -> Vec < Arg > { debug ! ("flags: name={}" , p . get_name ()) ; p . get_arguments () . filter (| a | ! a . get_num_args () . expect ("built") . takes_values () && ! a . is_positional ()) . cloned () . collect () }
    };
}

flags!();