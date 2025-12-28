macro_rules! deps {
    () => {
        NameClass!();
        NameRefClass!();
        OperatorClass!();
    };
}

macro_rules! IdentClass {
    () => {
        deps!();
        # [derive (Debug)] pub enum IdentClass < 'db > { NameClass (NameClass < 'db >) , NameRefClass (NameRefClass < 'db >) , Operator (OperatorClass) , }
    };
}

IdentClass!();