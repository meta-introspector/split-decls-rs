macro_rules! deps {
    () => {
        JacobiSymbol!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl From < JacobiSymbol > for i8 { fn from (symbol : JacobiSymbol) -> i8 { symbol as i8 } }
    };
}

impl_131!()