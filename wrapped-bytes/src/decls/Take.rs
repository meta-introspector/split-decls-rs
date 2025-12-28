macro_rules! deps {
    () => {
        Buf!();
    };
}

macro_rules! Take {
    () => {
        deps!();
        # [doc = " A `Buf` adapter which limits the bytes read from an underlying buffer."] # [doc = ""] # [doc = " This struct is generally created by calling `take()` on `Buf`. See"] # [doc = " documentation of [`take()`](Buf::take) for more details."] # [derive (Debug)] pub struct Take < T > { inner : T , limit : usize , }
    };
}

Take!()