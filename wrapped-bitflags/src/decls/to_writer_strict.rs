macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! to_writer_strict {
    () => {
        deps!();
        # [doc = "\nWrite only the contained, defined, named flags in a flags value as text.\n"] pub fn to_writer_strict < B : Flags > (flags : & B , mut writer : impl Write) -> Result < () , fmt :: Error > { let mut first = true ; let mut iter = flags . iter_names () ; for (name , _) in & mut iter { if ! first { writer . write_str (" | ") ? ; } first = false ; writer . write_str (name) ? ; } fmt :: Result :: Ok (()) }
    };
}

to_writer_strict!()