macro_rules! deps {
    () => {
        DB!();
        DBRawIteratorWithThreadMode!();
    };
}

macro_rules! DBRawIterator {
    () => {
        deps!();
        # [doc = " A type alias to keep compatibility. See [`DBRawIteratorWithThreadMode`] for details"] pub type DBRawIterator < 'a > = DBRawIteratorWithThreadMode < 'a , DB > ;
    };
}

DBRawIterator!();