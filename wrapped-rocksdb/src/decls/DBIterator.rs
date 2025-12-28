macro_rules! deps {
    () => {
        DBIteratorWithThreadMode!();
        DB!();
    };
}

macro_rules! DBIterator {
    () => {
        deps!();
        # [doc = " A type alias to keep compatibility. See [`DBIteratorWithThreadMode`] for details"] pub type DBIterator < 'a > = DBIteratorWithThreadMode < 'a , DB > ;
    };
}

DBIterator!();