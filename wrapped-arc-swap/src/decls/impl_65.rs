macro_rules! deps {
    () => {
        Debt!();
        Slots!();
        Handover!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl Default for Slots { fn default () -> Self { Slots { control : AtomicUsize :: new (IDLE) , slot : Debt :: default () , active_addr : AtomicUsize :: new (0) , handover : Handover :: default () , space_offer : AtomicPtr :: new (ptr :: null_mut ()) , } } }
    };
}

impl_65!();