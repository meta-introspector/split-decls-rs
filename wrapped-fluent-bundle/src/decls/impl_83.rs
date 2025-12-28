macro_rules! deps {
    () => {
        FluentNumber!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl From < & FluentNumber > for PluralOperands { fn from (input : & FluentNumber) -> Self { let mut operands : Self = input . value . try_into () . expect ("Failed to generate operands out of FluentNumber") ; if let Some (mfd) = input . options . minimum_fraction_digits { if mfd > operands . v { operands . f *= 10_u64 . pow (mfd as u32 - operands . v as u32) ; operands . v = mfd ; } } operands } }
    };
}

impl_83!();