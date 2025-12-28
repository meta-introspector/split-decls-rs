macro_rules! deps {
    () => {
        Error!();
        Rows!();
    };
}

macro_rules! AndThenRows {
    () => {
        deps!();
        # [doc = " An iterator over the mapped resulting rows of a query, with an Error type"] # [doc = " unifying with Error."] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct AndThenRows < 'stmt , F > { rows : Rows < 'stmt > , map : F , }
    };
}

AndThenRows!();