macro_rules! deps {
    () => {
        ConstContext!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl ConstContext { # [doc = " A description of this const context that can appear between backticks in an error message."] # [doc = ""] # [doc = " E.g. `const` or `static mut`."] pub fn keyword_name (self) -> & 'static str { match self { Self :: Const { .. } => "const" , Self :: Static (Mutability :: Not) => "static" , Self :: Static (Mutability :: Mut) => "static mut" , Self :: ConstFn => "const fn" , } } }
    };
}

impl_214!()