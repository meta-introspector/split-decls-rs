macro_rules! deps {
    () => {
        InputValueResult!();
        Any!();
        ScalarType!();
        Scalar!();
    };
}

macro_rules! impl_748 {
    () => {
        deps!();
        # [doc = " The `_Any` scalar is used to pass representations of entities from external"] # [doc = " services into the root `_entities` field for execution."] # [Scalar (internal , name = "_Any")] impl ScalarType for Any { fn parse (value : Value) -> InputValueResult < Self > { Ok (Self (value)) } fn is_valid (_value : & Value) -> bool { true } fn to_value (& self) -> Value { self . 0 . clone () } }
    };
}

impl_748!();