macro_rules! deps {
    () => {
        SectionKind!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl SectionKind { # [doc = " Return true if this section contains zerofill data."] pub fn is_bss (self) -> bool { self == SectionKind :: UninitializedData || self == SectionKind :: UninitializedTls || self == SectionKind :: Common } }
    };
}

impl_11!();