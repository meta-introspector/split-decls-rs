macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! MIN_DATE {
    () => {
        deps!();
        # [doc = " The minimum possible `NaiveDate` (January 1, 262145 BCE)."] # [deprecated (since = "0.4.20" , note = "Use NaiveDate::MIN instead")] pub const MIN_DATE : NaiveDate = NaiveDate :: MIN ;
    };
}

MIN_DATE!()