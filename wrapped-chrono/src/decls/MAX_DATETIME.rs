macro_rules! deps {
    () => {
        NaiveDateTime!();
    };
}

macro_rules! MAX_DATETIME {
    () => {
        deps!();
        # [doc = " The maximum possible `NaiveDateTime`."] # [deprecated (since = "0.4.20" , note = "Use NaiveDateTime::MAX instead")] pub const MAX_DATETIME : NaiveDateTime = NaiveDateTime :: MAX ;
    };
}

MAX_DATETIME!()