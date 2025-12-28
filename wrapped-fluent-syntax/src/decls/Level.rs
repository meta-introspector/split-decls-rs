macro_rules! deps {
    () => {
        Resource!();
    };
}

macro_rules! Level {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , PartialEq)] pub (super) enum Level { None = 0 , Regular = 1 , Group = 2 , Resource = 3 , }
    };
}

Level!();