macro_rules! deps {
    () => {
        State!();
        LexedStr!();
        StrStep!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        struct Builder < 'a , 'b > { lexed : & 'a LexedStr < 'a > , pos : usize , state : State , sink : & 'b mut dyn FnMut (StrStep < '_ >) , }
    };
}

Builder!()