macro_rules! deps {
    () => {
        GraphRef!();
        Reversed!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < G : GraphRef > GraphRef for Reversed < G > { }
    };
}

impl_170!()