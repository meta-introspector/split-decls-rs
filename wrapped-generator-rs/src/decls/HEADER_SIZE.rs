macro_rules! deps {
    () => {
        StackBoxHeader!();
    };
}

macro_rules! HEADER_SIZE {
    () => {
        deps!();
        const HEADER_SIZE : usize = std :: mem :: size_of :: < StackBoxHeader > () / std :: mem :: size_of :: < usize > () ;
    };
}

HEADER_SIZE!()