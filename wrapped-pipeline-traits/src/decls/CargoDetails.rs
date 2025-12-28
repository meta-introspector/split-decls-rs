macro_rules! deps {
    () => {
        CargoInfo!();
    };
}

macro_rules! CargoDetails {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum CargoDetails { Info (CargoInfo) , Error (String) , Unknown , }
    };
}

CargoDetails!();