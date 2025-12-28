macro_rules! check {
    () => {
        macro_rules ! check { ($ e : expr) => { $ e . expect ("Writing to a Vec should never fail.") } ; }
    };
}

check!();