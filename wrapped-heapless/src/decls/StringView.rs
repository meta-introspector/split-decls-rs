macro_rules! deps {
    () => {
        StringInner!();
        ViewStorage!();
    };
}

macro_rules! StringView {
    () => {
        deps!();
        # [doc = " A dynamic capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html)."] pub type StringView < LenT = usize > = StringInner < LenT , ViewStorage > ;
    };
}

StringView!()