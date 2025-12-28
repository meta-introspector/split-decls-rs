macro_rules! deps {
    () => {
        InlineAsmOptions!();
        AsmMacro!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl AsmMacro { pub const fn macro_name (self) -> & 'static str { match self { AsmMacro :: Asm => "asm" , AsmMacro :: GlobalAsm => "global_asm" , AsmMacro :: NakedAsm => "naked_asm" , } } pub const fn is_supported_option (self , option : InlineAsmOptions) -> bool { match self { AsmMacro :: Asm => true , AsmMacro :: GlobalAsm => InlineAsmOptions :: GLOBAL_OPTIONS . contains (option) , AsmMacro :: NakedAsm => InlineAsmOptions :: NAKED_OPTIONS . contains (option) , } } pub const fn diverges (self , options : InlineAsmOptions) -> bool { match self { AsmMacro :: Asm => options . contains (InlineAsmOptions :: NORETURN) , AsmMacro :: GlobalAsm => true , AsmMacro :: NakedAsm => true , } } }
    };
}

impl_154!()