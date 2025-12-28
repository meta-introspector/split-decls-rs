macro_rules! deps {
    () => {
        StateID!();
        Utf8Node!();
        Transition!();
    };
}

macro_rules! impl_479 {
    () => {
        deps!();
        impl Utf8Node { fn set_last_transition (& mut self , next : StateID) { if let Some (last) = self . last . take () { self . trans . push (Transition { start : last . start , end : last . end , next , }) ; } } }
    };
}

impl_479!()