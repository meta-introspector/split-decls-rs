macro_rules! deps {
    () => {
        LevelInner!();
        Level!();
    };
}

macro_rules! HELP {
    () => {
        deps!();
        # [doc = " Default `help:` [`Level`]"] pub const HELP : Level < '_ > = Level { name : None , level : LevelInner :: Help , } ;
    };
}

HELP!()