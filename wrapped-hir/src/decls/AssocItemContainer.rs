macro_rules! deps {
    () => {
        Trait!();
        Impl!();
    };
}

macro_rules! AssocItemContainer {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub enum AssocItemContainer { Trait (Trait) , Impl (Impl) , }
    };
}

AssocItemContainer!()