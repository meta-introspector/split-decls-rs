macro_rules! deps {
    () => {
        CountItem!();
        CoalesceBy!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < I , F , C > fmt :: Debug for CoalesceBy < I , F , C > where I : Iterator + fmt :: Debug , C : CountItem < I :: Item > , C :: CItem : fmt :: Debug , { debug_fmt_fields ! (CoalesceBy , iter , last) ; }
    };
}

impl_13!();