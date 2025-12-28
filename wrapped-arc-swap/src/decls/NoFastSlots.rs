macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! NoFastSlots {
    () => {
        deps!();
        # [doc = " Config for no fast slots."] # [derive (Clone , Copy , Default)] pub struct NoFastSlots ;
    };
}

NoFastSlots!();