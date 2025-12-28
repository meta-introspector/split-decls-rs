macro_rules! deps {
    () => {
        InputValueError!();
        Result!();
        InputType!();
    };
}

macro_rules! CustomValidator {
    () => {
        deps!();
        # [doc = " Represents a custom input value validator."] pub trait CustomValidator < T : InputType > { # [doc = " Check the value is valid."] fn check (& self , value : & T) -> Result < () , InputValueError < T > > ; }
    };
}

CustomValidator!();