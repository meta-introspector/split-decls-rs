macro_rules! deps {
    () => {
        ImplementType!();
    };
}

macro_rules! ImplementAttributes {
    () => {
        deps!();
        # [derive (Default)] struct ImplementAttributes { pub implement : Vec < ImplementType > , pub trust_level : usize , pub agile : bool , }
    };
}

ImplementAttributes!();