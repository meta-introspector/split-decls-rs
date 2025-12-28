macro_rules! HwCounterRead {
    () => {
        trait HwCounterRead { type Output ; fn read (& self) -> Self :: Output ; }
    };
}

HwCounterRead!();