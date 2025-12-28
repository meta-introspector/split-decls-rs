macro_rules! deps {
    () => {
        EndsWithNewline!();
        Interner!();
    };
}

macro_rules! BasicLineDiffPrinter {
    () => {
        deps!();
        pub struct BasicLineDiffPrinter < 'a , T : EndsWithNewline + ? Sized + Hash + Eq + Display > (pub & 'a Interner < & 'a T > ,) ;
    };
}

BasicLineDiffPrinter!()