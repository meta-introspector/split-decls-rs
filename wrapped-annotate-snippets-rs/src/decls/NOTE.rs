macro_rules! deps {
    () => {
        Level!();
        LevelInner!();
    };
}

macro_rules! NOTE {
    () => {
        deps!();
        # [doc = " Default `note:` [`Level`]"] pub const NOTE : Level < '_ > = Level { name : None , level : LevelInner :: Note , } ;
    };
}

NOTE!()