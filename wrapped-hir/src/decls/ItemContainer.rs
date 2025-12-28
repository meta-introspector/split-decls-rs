macro_rules! deps {
    () => {
        Trait!();
        Module!();
        Crate!();
        Impl!();
        ExternBlock!();
    };
}

macro_rules! ItemContainer {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ItemContainer { Trait (Trait) , Impl (Impl) , Module (Module) , ExternBlock (ExternBlock) , Crate (Crate) , }
    };
}

ItemContainer!()