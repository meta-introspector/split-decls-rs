macro_rules! macro_10 {
    () => {
        lazy_static ! { # [doc = " The terminfo database."] static ref TERMINFO : Option < Database > = Database :: from_env () . ok () ; }
    };
}

macro_10!()