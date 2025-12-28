macro_rules! deps {
    () => {
        MockFunction!();
    };
}

macro_rules! Matcher {
    () => {
        deps!();
        struct Matcher < 'a > { f : & 'a MockFunction }
    };
}

Matcher!()