macro_rules! deps {
    () => {
        NaiveDate!();
        NaiveDateTime!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl From < NaiveDateTime > for NaiveDate { fn from (naive_datetime : NaiveDateTime) -> Self { naive_datetime . date () } }
    };
}

impl_350!();