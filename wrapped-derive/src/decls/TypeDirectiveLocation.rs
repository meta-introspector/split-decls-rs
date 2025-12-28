macro_rules! deps {
    () => {
        Object!();
        Enum!();
        Interface!();
        InputObject!();
    };
}

macro_rules! TypeDirectiveLocation {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , FromMeta , strum :: Display)] # [darling (rename_all = "PascalCase")] # [strum (serialize_all = "SCREAMING_SNAKE_CASE")] pub enum TypeDirectiveLocation { ArgumentDefinition , Enum , EnumValue , FieldDefinition , InputFieldDefinition , Object , InputObject , Interface , }
    };
}

TypeDirectiveLocation!()