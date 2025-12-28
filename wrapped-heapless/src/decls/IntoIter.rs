macro_rules! deps {
    () => {
        Vec!();
        LenType!();
    };
}

macro_rules! IntoIter {
    () => {
        deps!();
        # [doc = " An iterator that moves out of an [`Vec`][`Vec`]."] # [doc = ""] # [doc = " This struct is created by calling the `into_iter` method on [`Vec`][`Vec`]."] pub struct IntoIter < T , const N : usize , LenT : LenType > { vec : Vec < T , N , LenT > , next : LenT , }
    };
}

IntoIter!();