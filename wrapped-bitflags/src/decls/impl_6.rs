macro_rules! deps {
    () => {
        IterNames!();
        Flag!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < B : 'static > IterNames < B > { # [doc (hidden)] pub const fn __private_const_new (flags : & 'static [Flag < B >] , source : B , remaining : B) -> Self { IterNames { flags , idx : 0 , remaining , source , } } # [doc = " Get a flags value of any remaining bits that haven't been yielded yet."] # [doc = ""] # [doc = " Once the iterator has finished, this method can be used to"] # [doc = " check whether or not there are any bits that didn't correspond"] # [doc = " to a contained, defined, named flag remaining."] pub fn remaining (& self) -> & B { & self . remaining } }
    };
}

impl_6!();