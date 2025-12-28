macro_rules! deps {
    () => {
        MultiThreaded!();
        DBWithThreadMode!();
    };
}

macro_rules! DB {
    () => {
        deps!();
        # [cfg (feature = "multi-threaded-cf")] pub type DB = DBWithThreadMode < MultiThreaded > ;
    };
}

DB!();