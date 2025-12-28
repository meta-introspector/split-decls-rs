macro_rules! deps {
    () => {
        MetaInputValue!();
    };
}

macro_rules! ArgumentsOfCorrectType {
    () => {
        deps!();
        # [derive (Default)] pub struct ArgumentsOfCorrectType < 'a > { current_args : Option < & 'a IndexMap < String , MetaInputValue > > , }
    };
}

ArgumentsOfCorrectType!();