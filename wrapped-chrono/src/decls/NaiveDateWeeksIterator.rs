macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! NaiveDateWeeksIterator {
    () => {
        deps!();
        # [doc = " Iterator over `NaiveDate` with a step size of one week."] # [derive (Debug , Copy , Clone , Hash , PartialEq , PartialOrd , Eq , Ord)] pub struct NaiveDateWeeksIterator { value : NaiveDate , }
    };
}

NaiveDateWeeksIterator!()