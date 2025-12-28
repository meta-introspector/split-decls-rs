macro_rules! deps {
    () => {
        Endian!();
        I32!();
        Rela32!();
        Rel32!();
    };
}

macro_rules! impl_1532 {
    () => {
        deps!();
        impl < E : Endian > From < Rel32 < E > > for Rela32 < E > { fn from (rel : Rel32 < E >) -> Self { Rela32 { r_offset : rel . r_offset , r_info : rel . r_info , r_addend : I32 :: default () , } } }
    };
}

impl_1532!();