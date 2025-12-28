macro_rules! Diff {
    () => {
        # [derive (Copy , Clone)] enum Diff < 'a , 'b > { Equal (Range < 'a > , Range < 'b >) , Delete (Range < 'a >) , Insert (Range < 'b >) , }
    };
}

Diff!()