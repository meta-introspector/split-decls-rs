macro_rules! deps {
    () => {
        LoopSource!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl LoopSource { pub fn name (self) -> & 'static str { match self { LoopSource :: Loop => "loop" , LoopSource :: While => "while" , LoopSource :: ForLoop => "for" , } } }
    };
}

impl_232!();