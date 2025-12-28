macro_rules! deps {
    () => {
        Receiver!();
        Result!();
        Upgraded!();
        Error!();
    };
}

macro_rules! OnUpgrade {
    () => {
        deps!();
        # [doc = " A future for a possible HTTP upgrade."] # [doc = ""] # [doc = " If no upgrade was available, or it doesn't succeed, yields an `Error`."] # [derive (Clone)] pub struct OnUpgrade { rx : Option < Arc < Mutex < oneshot :: Receiver < crate :: Result < Upgraded > > > > > , }
    };
}

OnUpgrade!()