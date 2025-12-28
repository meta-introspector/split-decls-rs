macro_rules! deps {
    () => {
        UnwindContextStorage!();
        ReaderOffset!();
        UnwindTableRow!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < T , S > Default for UnwindTableRow < T , S > where T : ReaderOffset , S : UnwindContextStorage < T > , { fn default () -> Self { UnwindTableRow { start_address : 0 , end_address : 0 , saved_args_size : 0 , cfa : Default :: default () , registers : Default :: default () , } } }
    };
}

impl_234!()