macro_rules! deps {
    () => {
        Crate!();
        Module!();
        Impl!();
        ExternBlock!();
        Trait!();
    };
}

macro_rules! ItemContainer {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ItemContainer { Trait (Trait) , Impl (Impl) , Module (Module) , ExternBlock (ExternBlock) , Crate (Crate) , }
    };
}

ItemContainer!()