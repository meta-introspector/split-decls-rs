macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! simple_id_construction {
    () => {
        deps!();
        # [test] fn simple_id_construction () { let id1 = Id :: new ("hello") ; match id1 { Ok (_) => { } Err (..) => panic ! ("'hello' is not a valid value for id anymore") , } }
    };
}

simple_id_construction!();