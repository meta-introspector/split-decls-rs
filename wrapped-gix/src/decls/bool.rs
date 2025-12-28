macro_rules! bool {
    () => {
        fn bool (v : bool) -> & 'static str { match v { true => "true" , false => "false" , } }
    };
}

bool!()