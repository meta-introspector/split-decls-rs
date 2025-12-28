macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! FinalState {
    () => {
        deps!();
        enum FinalState < E = () > { Pending , AllDone , Error (E) , }
    };
}

FinalState!()