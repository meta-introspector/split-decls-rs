macro_rules! deps {
    () => {
        TypeDef!();
    };
}

macro_rules! InterfaceImpl {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub struct InterfaceImpl { pub Class : id :: TypeDef , pub Interface : TypeDefOrRef , }
    };
}

InterfaceImpl!();