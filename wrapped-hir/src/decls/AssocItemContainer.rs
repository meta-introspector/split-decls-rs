macro_rules! deps {
    () => {
        Impl!();
        Trait!();
    };
}

macro_rules! AssocItemContainer {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum AssocItemContainer { Trait (Trait) , Impl (Impl) , }
    };
}

AssocItemContainer!();