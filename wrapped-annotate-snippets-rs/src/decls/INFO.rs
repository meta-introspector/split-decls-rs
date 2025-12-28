macro_rules! deps {
    () => {
        Level!();
        LevelInner!();
    };
}

macro_rules! INFO {
    () => {
        deps!();
        # [doc = " Default `info:` [`Level`]"] pub const INFO : Level < '_ > = Level { name : None , level : LevelInner :: Info , } ;
    };
}

INFO!();