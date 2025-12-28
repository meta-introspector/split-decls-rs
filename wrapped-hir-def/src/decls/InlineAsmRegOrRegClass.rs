macro_rules! InlineAsmRegOrRegClass {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Hash)] pub enum InlineAsmRegOrRegClass { Reg (Symbol) , RegClass (Symbol) , }
    };
}

InlineAsmRegOrRegClass!()