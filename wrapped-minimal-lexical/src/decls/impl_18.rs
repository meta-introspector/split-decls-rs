macro_rules! deps {
    () => {
        ReverseView!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'a , T > ops :: Index < usize > for ReverseView < 'a , T > { type Output = T ; # [inline] fn index (& self , index : usize) -> & T { let len = self . inner . len () ; & (* self . inner) [len - index - 1] } }
    };
}

impl_18!();