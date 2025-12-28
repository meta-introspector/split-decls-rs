macro_rules! deps {
    () => {
        TypesItem!();
        ValuesItem!();
        MacrosItem!();
    };
}

macro_rules! PerNs {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , Default , Eq , Hash , PartialEq)] pub struct PerNs { pub types : Option < TypesItem > , pub values : Option < ValuesItem > , pub macros : Option < MacrosItem > , }
    };
}

PerNs!();