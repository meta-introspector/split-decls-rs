macro_rules! deps {
    () => {
        Operation!();
        Push!();
        Instruction!();
        Fetch!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Instruction < '_ > { # [doc = " Derive the mode of operation from this instruction."] pub fn operation (& self) -> Operation { match self { Instruction :: Push (_) => Operation :: Push , Instruction :: Fetch (_) => Operation :: Fetch , } } }
    };
}

impl_5!();