macro_rules! deps {
    () => {
        Type!();
        InterfaceKind!();
    };
}

macro_rules! Interface {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct Interface { pub def : TypeDef , pub generics : Vec < Type > , pub kind : InterfaceKind , }
    };
}

Interface!();