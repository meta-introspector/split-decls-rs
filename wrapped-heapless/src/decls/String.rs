macro_rules! deps {
    () => {
        StringInner!();
        OwnedStorage!();
    };
}

macro_rules! String {
    () => {
        deps!();
        # [doc = " A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html)."] pub type String < const N : usize , LenT = usize > = StringInner < LenT , OwnedStorage < N > > ;
    };
}

String!();