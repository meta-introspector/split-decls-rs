macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! Diff {
    () => {
        deps!();
        # [derive (Copy , Clone)] enum Diff < 'a , 'b > { Equal (Range < 'a > , Range < 'b >) , Delete (Range < 'a >) , Insert (Range < 'b >) , }
    };
}

Diff!()