macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! property {
    () => {
        deps!();
        macro_rules ! property { ($ suffix : literal) => { PropName :: new_unwrap (concat ! ("rocksdb." , $ suffix , "\0")) } ; }
    };
}

property!();