macro_rules! deps {
    () => {
        Level!();
        LevelInner!();
    };
}

macro_rules! ERROR {
    () => {
        deps!();
        # [doc = " Default `error:` [`Level`]"] pub const ERROR : Level < '_ > = Level { name : None , level : LevelInner :: Error , } ;
    };
}

ERROR!();