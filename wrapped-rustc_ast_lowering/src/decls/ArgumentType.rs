macro_rules! ArgumentType {
    () => {
        # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq)] enum ArgumentType { Format (FormatTrait) , Usize , }
    };
}

ArgumentType!();