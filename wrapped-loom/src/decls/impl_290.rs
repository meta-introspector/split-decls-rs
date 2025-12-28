macro_rules! deps {
    () => {
        Arc!();
        Sender!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < T > Clone for Sender < T > { fn clone (& self) -> Sender < T > { Sender { object : std :: sync :: Arc :: clone (& self . object) , sender : self . sender . clone () , } } }
    };
}

impl_290!()