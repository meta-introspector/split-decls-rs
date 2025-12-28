macro_rules! deps {
    () => {
        VariableDefinition!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl VariableDefinition { # [doc = " Get the default value of the variable; this is `default_value` if it is"] # [doc = " present, `Value::Null` if it is nullable and `None` otherwise."] # [must_use] pub fn default_value (& self) -> Option < & ConstValue > { self . default_value . as_ref () . map (| value | & value . node) . or ({ if self . var_type . node . nullable { Some (& ConstValue :: Null) } else { None } }) } }
    };
}

impl_10!()