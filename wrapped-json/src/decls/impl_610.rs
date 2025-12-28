macro_rules! deps {
    () => {
        RawValue!();
        Formatter!();
        Result!();
    };
}

macro_rules! impl_610 {
    () => {
        deps!();
        impl Debug for RawValue { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . debug_tuple ("RawValue") . field (& format_args ! ("{}" , & self . json)) . finish () } }
    };
}

impl_610!()