macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! UnusedVariable {
    () => {
        deps!();
        # [derive (Debug)] pub struct UnusedVariable { pub local : Local , }
    };
}

UnusedVariable!();