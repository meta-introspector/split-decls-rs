macro_rules! macro_52 {
    () => {
        std :: thread_local ! (static ENTERED : Cell < bool > = const { Cell :: new (false) }) ;
    };
}

macro_52!()