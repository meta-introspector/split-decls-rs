macro_rules! deps {
    () => {
        PathList!();
        GenericParamList!();
    };
}

macro_rules! ConcreteType {
    () => {
        deps!();
        # [derive (FromMeta)] pub struct ConcreteType { pub name : String , # [darling (default)] pub input_name : Option < String > , pub params : PathList , # [darling (default)] pub bounds : GenericParamList , }
    };
}

ConcreteType!()