macro_rules! deps {
    () => {
        Rel64!();
        Endian!();
        I64!();
        Rela64!();
    };
}

macro_rules! impl_1535 {
    () => {
        deps!();
        impl < E : Endian > From < Rel64 < E > > for Rela64 < E > { fn from (rel : Rel64 < E >) -> Self { Rela64 { r_offset : rel . r_offset , r_info : rel . r_info , r_addend : I64 :: default () , } } }
    };
}

impl_1535!();