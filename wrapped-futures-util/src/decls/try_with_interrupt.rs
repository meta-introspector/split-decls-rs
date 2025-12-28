macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! try_with_interrupt {
    () => {
        deps!();
        macro_rules ! try_with_interrupt { ($ e : expr) => { loop { match $ e { Ok (e) => { break e ; } Err (ref e) if e . kind () == :: std :: io :: ErrorKind :: Interrupted => { continue ; } Err (e) => { return Poll :: Ready (Err (e)) ; } } } } ; }
    };
}

try_with_interrupt!();