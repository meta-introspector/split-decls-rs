macro_rules! InlineAsmKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum InlineAsmKind { # [doc = " `asm!()`."] Asm , # [doc = " `global_asm!()`."] GlobalAsm , # [doc = " `naked_asm!()`."] NakedAsm , }
    };
}

InlineAsmKind!()