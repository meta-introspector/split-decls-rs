macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! AsmMacro {
    () => {
        deps!();
        # [derive (Clone , Copy , Encodable , Decodable , Debug , HashStable_Generic , Walkable , PartialEq , Eq)] pub enum AsmMacro { # [doc = " The `asm!` macro"] Asm , # [doc = " The `global_asm!` macro"] GlobalAsm , # [doc = " The `naked_asm!` macro"] NakedAsm , }
    };
}

AsmMacro!();