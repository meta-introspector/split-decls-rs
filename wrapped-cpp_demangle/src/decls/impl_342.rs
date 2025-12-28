macro_rules! deps {
    () => {
        ParseOptions!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl ParseOptions { # [doc = " Set the limit on recursion depth during the parsing phase. A low"] # [doc = " limit will cause valid symbols to be rejected, but a high limit may"] # [doc = " allow pathological symbols to overflow the stack during parsing."] # [doc = " The default value is 96, which will not overflow the stack even in"] # [doc = " a debug build."] pub fn recursion_limit (mut self , limit : u32) -> Self { self . recursion_limit = Some (NonZeroU32 :: new (limit) . expect ("Recursion limit must be > 0")) ; self } }
    };
}

impl_342!();