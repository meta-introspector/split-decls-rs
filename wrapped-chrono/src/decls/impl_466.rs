macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl fmt :: Debug for YearFlags { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let YearFlags (flags) = * self ; match flags { 0o15 => "A" . fmt (f) , 0o05 => "AG" . fmt (f) , 0o14 => "B" . fmt (f) , 0o04 => "BA" . fmt (f) , 0o13 => "C" . fmt (f) , 0o03 => "CB" . fmt (f) , 0o12 => "D" . fmt (f) , 0o02 => "DC" . fmt (f) , 0o11 => "E" . fmt (f) , 0o01 => "ED" . fmt (f) , 0o10 => "F?" . fmt (f) , 0o00 => "FE?" . fmt (f) , 0o17 => "F" . fmt (f) , 0o07 => "FE" . fmt (f) , 0o16 => "G" . fmt (f) , 0o06 => "GF" . fmt (f) , _ => write ! (f , "YearFlags({flags})") , } } }
    };
}

impl_466!();