macro_rules! deps {
    () => {
        VecExt!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < T > VecExt < T > for Vec < T > { fn retain_mut < F > (& mut self , mut f : F) where F : FnMut (& mut T) -> bool , { let len = self . len () ; let mut del = 0 ; { let v = & mut * * self ; for i in 0 .. len { if ! f (& mut v [i]) { del += 1 ; } else if del > 0 { v . swap (i - del , i) ; } } } if del > 0 { self . truncate (len - del) ; } } }
    };
}

impl_153!();