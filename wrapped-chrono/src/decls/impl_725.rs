macro_rules! deps {
    () => {
        OutOfRange!();
        Error!();
        Month!();
    };
}

macro_rules! impl_725 {
    () => {
        deps!();
        impl TryFrom < u8 > for Month { type Error = OutOfRange ; fn try_from (value : u8) -> Result < Self , Self :: Error > { match value { 1 => Ok (Month :: January) , 2 => Ok (Month :: February) , 3 => Ok (Month :: March) , 4 => Ok (Month :: April) , 5 => Ok (Month :: May) , 6 => Ok (Month :: June) , 7 => Ok (Month :: July) , 8 => Ok (Month :: August) , 9 => Ok (Month :: September) , 10 => Ok (Month :: October) , 11 => Ok (Month :: November) , 12 => Ok (Month :: December) , _ => Err (OutOfRange :: new ()) , } } }
    };
}

impl_725!();