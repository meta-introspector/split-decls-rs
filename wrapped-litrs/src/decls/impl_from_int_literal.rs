macro_rules! deps {
    () => {
        IntegerType!();
        FromIntegerLiteral!();
    };
}

macro_rules! impl_from_int_literal {
    () => {
        deps!();
        macro_rules ! impl_from_int_literal { ($ ($ ty : ty => $ variant : ident ,) *) => { $ (impl self :: sealed :: Sealed for $ ty { } impl FromIntegerLiteral for $ ty { fn from_small_number (n : u8) -> Self { n as Self } fn checked_add (self , rhs : Self) -> Option < Self > { self . checked_add (rhs) } fn checked_mul (self , rhs : Self) -> Option < Self > { self . checked_mul (rhs) } fn ty () -> IntegerType { IntegerType ::$ variant } }) * } ; }
    };
}

impl_from_int_literal!()