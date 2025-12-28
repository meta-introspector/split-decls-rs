macro_rules! deps {
    () => {
        StackBoxHeader!();
    };
}

macro_rules! ALIGN {
    () => {
        deps!();
        const ALIGN : usize = std :: mem :: size_of :: < StackBoxHeader > () ;
    };
}

ALIGN!();