macro_rules! deps {
    () => {
        Weekday!();
    };
}

macro_rules! impl_700 {
    () => {
        deps!();
        # [doc = " Any weekday can be represented as an integer from 0 to 6, which equals to"] # [doc = " [`Weekday::num_days_from_monday`](#method.num_days_from_monday) in this implementation."] # [doc = " Do not heavily depend on this though; use explicit methods whenever possible."] impl num_traits :: FromPrimitive for Weekday { # [inline] fn from_i64 (n : i64) -> Option < Weekday > { match n { 0 => Some (Weekday :: Mon) , 1 => Some (Weekday :: Tue) , 2 => Some (Weekday :: Wed) , 3 => Some (Weekday :: Thu) , 4 => Some (Weekday :: Fri) , 5 => Some (Weekday :: Sat) , 6 => Some (Weekday :: Sun) , _ => None , } } # [inline] fn from_u64 (n : u64) -> Option < Weekday > { match n { 0 => Some (Weekday :: Mon) , 1 => Some (Weekday :: Tue) , 2 => Some (Weekday :: Wed) , 3 => Some (Weekday :: Thu) , 4 => Some (Weekday :: Fri) , 5 => Some (Weekday :: Sat) , 6 => Some (Weekday :: Sun) , _ => None , } } }
    };
}

impl_700!()