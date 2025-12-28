macro_rules! deps {
    () => {
        Allocation!();
        State!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Allocation { pub (crate) fn new (location : Location) -> Allocation { rt :: execution (| execution | { let state = execution . objects . insert (State { is_dropped : false , allocated : location , }) ; trace ! (? state , % location , "Allocation::new") ; Allocation { state } }) } }
    };
}

impl_19!();