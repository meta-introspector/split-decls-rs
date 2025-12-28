macro_rules! deps {
    () => {
        Operation!();
    };
}

macro_rules! Expression {
    () => {
        deps!();
        # [doc = " The bytecode for a DWARF expression or location description."] # [derive (Debug , Default , Clone , PartialEq , Eq , Hash)] pub struct Expression { operations : Vec < Operation > , }
    };
}

Expression!();