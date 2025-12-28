macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! Collect {
    () => {
        deps!();
        struct Collect { # [cfg (feature = "parallel")] tx : std :: sync :: mpsc :: Sender < Item > , # [cfg (not (feature = "parallel"))] items : Vec < Item > , }
    };
}

Collect!();