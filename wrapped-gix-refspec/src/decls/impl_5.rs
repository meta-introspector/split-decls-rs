macro_rules! deps {
    () => {
        Fetch!();
        Instruction!();
        Push!();
        Operation!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Instruction < '_ > { # [doc = " Derive the mode of operation from this instruction."] pub fn operation (& self) -> Operation { match self { Instruction :: Push (_) => Operation :: Push , Instruction :: Fetch (_) => Operation :: Fetch , } } }
    };
}

impl_5!()