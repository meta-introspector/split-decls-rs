macro_rules! deps {
    () => {
        Vec!();
        LenType!();
    };
}

macro_rules! CString {
    () => {
        deps!();
        # [doc = " A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html)."] # [doc = ""] # [doc = " It stores up to `N - 1` non-nul characters with a trailing nul terminator."] # [derive (Clone , Hash)] pub struct CString < const N : usize , LenT : LenType = usize > { inner : Vec < u8 , N , LenT > , }
    };
}

CString!()