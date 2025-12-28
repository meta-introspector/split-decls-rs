macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! References {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct References (Vec < Reference >) ;
    };
}

References!()