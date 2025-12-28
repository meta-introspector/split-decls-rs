macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [doc = " # Constructors"] impl < 'a > Level < 'a > { pub const ERROR : Level < 'a > = ERROR ; pub const WARNING : Level < 'a > = WARNING ; pub const INFO : Level < 'a > = INFO ; pub const NOTE : Level < 'a > = NOTE ; pub const HELP : Level < 'a > = HELP ; }
    };
}

impl_6!()