macro_rules! deps {
    () => {
        StrConsumer!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [doc = " Pushes the str onto the end of the String"] impl StrConsumer for String { fn consume (& mut self , buf : & str) { self . push_str (buf) ; } }
    };
}

impl_55!();