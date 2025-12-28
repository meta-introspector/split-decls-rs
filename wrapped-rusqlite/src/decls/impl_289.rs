macro_rules! deps {
    () => {
        ConnRef!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl ConnRef < '_ > { # [doc = " Test for auto-commit mode."] pub fn is_autocommit (& self) -> bool { unsafe { crate :: inner_connection :: get_autocommit (self . ptr) } } # [doc = " the path to the database file, if one exists and is known."] pub fn db_filename (& self) -> Option < & str > { unsafe { crate :: inner_connection :: db_filename (self . phantom , self . ptr , MAIN_DB) } } }
    };
}

impl_289!()