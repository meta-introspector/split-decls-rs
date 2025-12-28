macro_rules! Version {
    () => {
        # [doc = " Structure representing a gnuplot version number."] pub struct Version { # [doc = " The major version number"] pub major : usize , # [doc = " The minor version number"] pub minor : usize , # [doc = " The patch level"] pub patch : String , }
    };
}

Version!();