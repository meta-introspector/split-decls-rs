macro_rules! deps {
    () => {
        State!();
        StrStep!();
        LexedStr!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        struct Builder < 'a , 'b > { lexed : & 'a LexedStr < 'a > , pos : usize , state : State , sink : & 'b mut dyn FnMut (StrStep < '_ >) , }
    };
}

Builder!();