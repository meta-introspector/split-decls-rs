macro_rules! deps {
    () => {
        UnwindExpression!();
        Reader!();
        UnwindSection!();
        Result!();
        Expression!();
        ReaderOffset!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < T : ReaderOffset > UnwindExpression < T > { # [doc = " Get the expression from the section."] # [doc = ""] # [doc = " The offset and length were previously validated when the"] # [doc = " `UnwindExpression` was created, so this should not fail."] pub fn get < R , S > (& self , section : & S) -> Result < Expression < R > > where R : Reader < Offset = T > , S : UnwindSection < R > , { let input = & mut section . section () . clone () ; input . skip (self . offset) ? ; let data = input . split (self . length) ? ; Ok (Expression (data)) } }
    };
}

impl_249!();