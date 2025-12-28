macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! Display {
    () => {
        deps!();
        struct Display < 'a > { name : & 'a Name , edition : Edition , }
    };
}

Display!();