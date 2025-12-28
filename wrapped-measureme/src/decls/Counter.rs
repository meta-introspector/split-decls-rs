macro_rules! deps {
    () => {
        InstructionsMinusIrqs!();
        Instructions!();
        InstructionsMinusRaw0420!();
        WallTime!();
    };
}

macro_rules! Counter {
    () => {
        deps!();
        pub enum Counter { WallTime (WallTime) , Instructions (Instructions) , InstructionsMinusIrqs (InstructionsMinusIrqs) , InstructionsMinusRaw0420 (InstructionsMinusRaw0420) , }
    };
}

Counter!()