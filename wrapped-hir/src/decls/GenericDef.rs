macro_rules! deps {
    () => {
        Impl!();
        Const!();
        TypeAlias!();
        Function!();
        Adt!();
        Trait!();
        Static!();
    };
}

macro_rules! GenericDef {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub enum GenericDef { Function (Function) , Adt (Adt) , Trait (Trait) , TypeAlias (TypeAlias) , Impl (Impl) , Const (Const) , Static (Static) , }
    };
}

GenericDef!()