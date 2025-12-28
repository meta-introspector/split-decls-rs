macro_rules! deps {
    () => {
        Word!();
    };
}

macro_rules! WORD_BYTES {
    () => {
        deps!();
        const WORD_BYTES : usize = size_of :: < Word > () ;
    };
}

WORD_BYTES!()