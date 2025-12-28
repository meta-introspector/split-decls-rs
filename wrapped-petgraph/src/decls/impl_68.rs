macro_rules! deps {
    () => {
        GraphRef!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < G > GraphRef for & G where G : GraphBase { }
    };
}

impl_68!()