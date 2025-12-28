macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [doc = " The linked list kind: min-list or max-list"] pub trait Kind : private :: Sealed { # [doc (hidden)] fn ordering () -> Ordering ; }
    };
}

Kind!()