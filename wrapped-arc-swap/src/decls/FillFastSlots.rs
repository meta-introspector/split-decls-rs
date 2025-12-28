macro_rules! deps {
    () => {
        HybridStrategy!();
        NoFastSlots!();
    };
}

macro_rules! FillFastSlots {
    () => {
        deps!();
        # [doc = " A strategy that fills the slots with some crap to make sure we test the fallbacks too."] # [deprecated (note = "Only for internal testing. Do not use")] pub type FillFastSlots = HybridStrategy < NoFastSlots > ;
    };
}

FillFastSlots!()