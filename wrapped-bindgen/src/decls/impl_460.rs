macro_rules! deps {
    () => {
        Interface!();
        Class!();
        Category!();
        Delegate!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl Category { fn new (def : TypeDef) -> Self { if let Some (extends) = def . extends () { if extends . namespace () == "System" { match extends . name () { "Enum" => Self :: Enum , "MulticastDelegate" => Self :: Delegate , "ValueType" => Self :: Struct , "Attribute" => Self :: Attribute , _ => Self :: Class , } } else { Self :: Class } } else { Self :: Interface } } }
    };
}

impl_460!();