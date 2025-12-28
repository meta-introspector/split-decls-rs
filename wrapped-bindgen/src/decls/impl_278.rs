macro_rules! deps {
    () => {
        Param!();
        ParamHint!();
        Value!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl From < & Param > for ParamHint { fn from (param : & Param) -> Self { for attribute in param . def . attributes () { match attribute . name () { "NativeArrayInfoAttribute" => { for (_ , value) in attribute . args () { match value { Value :: I16 (value) => return Self :: ArrayRelativeLen (value as usize) , Value :: I32 (value) => return Self :: ArrayFixed (value as usize) , _ => { } } } } "MemorySizeAttribute" => { for (_ , value) in attribute . args () { if let Value :: I16 (value) = value { return Self :: ArrayRelativeByteLen (value as usize) ; } } } _ => { } } } Self :: None } }
    };
}

impl_278!();