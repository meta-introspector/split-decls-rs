macro_rules! deps {
    () => {
        PadUsing!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        impl < I , F > std :: fmt :: Debug for PadUsing < I , F > where I : std :: fmt :: Debug , { debug_fmt_fields ! (PadUsing , iter , min , pos) ; }
    };
}

impl_385!();