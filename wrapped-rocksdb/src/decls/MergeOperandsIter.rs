macro_rules! deps {
    () => {
        MergeOperands!();
    };
}

macro_rules! MergeOperandsIter {
    () => {
        deps!();
        pub struct MergeOperandsIter < 'a > { operands : & 'a MergeOperands , cursor : usize , }
    };
}

MergeOperandsIter!();