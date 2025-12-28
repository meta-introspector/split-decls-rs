macro_rules! deps {
    () => {
        UnifiedDiffConfig!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl UnifiedDiffConfig { pub fn context_len (& mut self , len : u32) -> & mut Self { self . context_len = len ; self } }
    };
}

impl_77!()