macro_rules! deps {
    () => {
        Trait!();
        Static!();
        Const!();
        TypeAlias!();
        Adt!();
        Impl!();
        Function!();
    };
}

macro_rules! GenericDef {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub enum GenericDef { Function (Function) , Adt (Adt) , Trait (Trait) , TypeAlias (TypeAlias) , Impl (Impl) , Const (Const) , Static (Static) , }
    };
}

GenericDef!()