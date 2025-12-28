macro_rules! deps {
    () => {
        NaiveDateTime!();
    };
}

macro_rules! MIN_DATETIME {
    () => {
        deps!();
        # [doc = " The minimum possible `NaiveDateTime`."] # [deprecated (since = "0.4.20" , note = "Use NaiveDateTime::MIN instead")] pub const MIN_DATETIME : NaiveDateTime = NaiveDateTime :: MIN ;
    };
}

MIN_DATETIME!()