macro_rules! deps {
    () => {
        Null!();
    };
}

macro_rules! macro_437 {
    () => {
        deps!();
        to_sql_self ! (Null) ;
    };
}

macro_437!();