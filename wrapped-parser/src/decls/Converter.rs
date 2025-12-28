macro_rules! deps {
    () => {
        LexedStr!();
    };
}

macro_rules! Converter {
    () => {
        deps!();
        struct Converter < 'a > { res : LexedStr < 'a > , offset : usize , edition : Edition , }
    };
}

Converter!();