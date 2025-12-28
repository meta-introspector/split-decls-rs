macro_rules! deps {
    () => {
        UncheckedOptionExt!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T > UncheckedOptionExt < T > for Option < T > { # [inline] unsafe fn unchecked_unwrap (self) -> T { match self { Some (x) => x , None => unreachable () , } } }
    };
}

impl_51!()