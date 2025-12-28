macro_rules! deps {
    () => {
        UnwindTableRow!();
        UnwindContextStorage!();
        ReaderOffset!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < T , S > Clone for UnwindTableRow < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn clone (& self) -> Self { Self { start_address : self . start_address , end_address : self . end_address , saved_args_size : self . saved_args_size , cfa : self . cfa . clone () , registers : self . registers . clone () , } } }
    };
}

impl_233!();