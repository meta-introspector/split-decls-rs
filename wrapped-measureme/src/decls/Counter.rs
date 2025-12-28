macro_rules! deps {
    () => {
        InstructionsMinusRaw0420!();
        InstructionsMinusIrqs!();
        Instructions!();
        WallTime!();
    };
}

macro_rules! Counter {
    () => {
        deps!();
        pub enum Counter { WallTime (WallTime) , Instructions (Instructions) , InstructionsMinusIrqs (InstructionsMinusIrqs) , InstructionsMinusRaw0420 (InstructionsMinusRaw0420) , }
    };
}

Counter!();