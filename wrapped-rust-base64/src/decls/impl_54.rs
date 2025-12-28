macro_rules! deps {
    () => {
        StrConsumer!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [doc = " As for `io::Write`, `StrConsumer` is implemented automatically for `&mut S`."] impl < S : StrConsumer + ? Sized > StrConsumer for & mut S { fn consume (& mut self , buf : & str) { (* * self) . consume (buf) ; } }
    };
}

impl_54!()