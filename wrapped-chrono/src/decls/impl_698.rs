macro_rules! deps {
    () => {
        Weekday!();
    };
}

macro_rules! impl_698 {
    () => {
        deps!();
        impl fmt :: Display for Weekday { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . pad (match * self { Weekday :: Mon => "Mon" , Weekday :: Tue => "Tue" , Weekday :: Wed => "Wed" , Weekday :: Thu => "Thu" , Weekday :: Fri => "Fri" , Weekday :: Sat => "Sat" , Weekday :: Sun => "Sun" , }) } }
    };
}

impl_698!()