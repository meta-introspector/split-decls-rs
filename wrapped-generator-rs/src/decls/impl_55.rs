macro_rules! deps {
    () => {
        Func!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Func { pub fn call_once (mut self) { let data = self . data ; self . data = ptr :: null_mut () ; (self . func) (data) ; } }
    };
}

impl_55!();