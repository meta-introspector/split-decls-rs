macro_rules! deps {
    () => {
        SalsaAttr!();
    };
}

macro_rules! filter_attrs {
    () => {
        deps!();
        fn filter_attrs (attrs : Vec < Attribute >) -> (Vec < Attribute > , Vec < SalsaAttr >) { let mut other = vec ! [] ; let mut salsa = vec ! [] ; for attr in attrs { match SalsaAttr :: try_from (attr) { Ok (it) => salsa . push (it) , Err (it) => other . push (it) , } } (other , salsa) }
    };
}

filter_attrs!()