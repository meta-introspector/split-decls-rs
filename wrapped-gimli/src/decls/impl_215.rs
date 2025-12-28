macro_rules! deps {
    () => {
        UnwindContextStorage!();
        ReaderOffset!();
        Result!();
        UnwindContext!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < T , S > Debug for UnwindContext < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("UnwindContext") . field ("stack" , & self . stack) . field ("initial_rule" , & self . initial_rule) . field ("is_initialized" , & self . is_initialized) . finish () } }
    };
}

impl_215!();