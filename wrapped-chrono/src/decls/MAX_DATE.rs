macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! MAX_DATE {
    () => {
        deps!();
        # [doc = " The maximum possible `NaiveDate` (December 31, 262143 CE)."] # [deprecated (since = "0.4.20" , note = "Use NaiveDate::MAX instead")] pub const MAX_DATE : NaiveDate = NaiveDate :: MAX ;
    };
}

MAX_DATE!();