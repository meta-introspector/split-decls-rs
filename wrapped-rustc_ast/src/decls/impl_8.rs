macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl PartialEq < & [Symbol] > for Path { # [inline] fn eq (& self , names : & & [Symbol]) -> bool { self . segments . len () == names . len () && self . segments . iter () . zip (names . iter ()) . all (| (s1 , s2) | s1 == s2) } }
    };
}

impl_8!()