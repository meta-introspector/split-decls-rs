macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! NaiveDateDaysIterator {
    () => {
        deps!();
        # [doc = " Iterator over `NaiveDate` with a step size of one day."] # [derive (Debug , Copy , Clone , Hash , PartialEq , PartialOrd , Eq , Ord)] pub struct NaiveDateDaysIterator { value : NaiveDate , }
    };
}

NaiveDateDaysIterator!();