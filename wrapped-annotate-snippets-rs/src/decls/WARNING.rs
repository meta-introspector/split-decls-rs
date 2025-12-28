macro_rules! deps {
    () => {
        Level!();
        LevelInner!();
    };
}

macro_rules! WARNING {
    () => {
        deps!();
        # [doc = " Default `warning:` [`Level`]"] pub const WARNING : Level < '_ > = Level { name : None , level : LevelInner :: Warning , } ;
    };
}

WARNING!();