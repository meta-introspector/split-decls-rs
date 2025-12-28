macro_rules! deps {
    () => {
        Unescape!();
    };
}

macro_rules! EscapeContainer {
    () => {
        deps!();
        pub (crate) trait EscapeContainer { fn new () -> Self ; fn is_empty (& self) -> bool ; fn push (& mut self , v : Unescape) ; fn push_str (& mut self , s : & str) ; }
    };
}

EscapeContainer!();