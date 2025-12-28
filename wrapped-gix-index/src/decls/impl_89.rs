macro_rules! deps {
    () => {
        Stage!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl From < Stage > for Flags { fn from (value : Stage) -> Self { Flags :: from_stage (value) } }
    };
}

impl_89!()