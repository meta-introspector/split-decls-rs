macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! badly_formatted_id {
    () => {
        deps!();
        # [test] fn badly_formatted_id () { let id2 = Id :: new ("Weird { struct : ure } !!!") ; match id2 { Ok (_) => panic ! ("graphviz id suddenly allows spaces, brackets and stuff") , Err (..) => { } } }
    };
}

badly_formatted_id!();