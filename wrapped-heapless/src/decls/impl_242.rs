macro_rules! deps {
    () => {
        LenType!();
        StringInner!();
    };
}

macro_rules! impl_242 {
    () => {
        deps!();
        impl < LenT : LenType , S : StringStorage + ? Sized > fmt :: Write for StringInner < LenT , S > { fn write_str (& mut self , s : & str) -> Result < () , fmt :: Error > { self . push_str (s) . map_err (| _ | fmt :: Error) } fn write_char (& mut self , c : char) -> Result < () , fmt :: Error > { self . push (c) . map_err (| _ | fmt :: Error) } }
    };
}

impl_242!();