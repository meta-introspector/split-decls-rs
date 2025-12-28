macro_rules! deps {
    () => {
        StateID!();
        Transition!();
        Utf8Node!();
    };
}

macro_rules! impl_479 {
    () => {
        deps!();
        impl Utf8Node { fn set_last_transition (& mut self , next : StateID) { if let Some (last) = self . last . take () { self . trans . push (Transition { start : last . start , end : last . end , next , }) ; } } }
    };
}

impl_479!();