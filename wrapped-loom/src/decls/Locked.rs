macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! Locked {
    () => {
        deps!();
        # [derive (Debug , PartialEq)] enum Locked { Read (HashSet < thread :: Id >) , Write (thread :: Id) , }
    };
}

Locked!()