macro_rules! deps {
    () => {
        Lazy!();
        Ready!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < F , R > Future for Lazy < F > where F : FnOnce (& mut Context < '_ >) -> R , { type Output = R ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < R > { Poll :: Ready ((self . f . take () . expect ("Lazy polled after completion")) (cx)) } }
    };
}

impl_150!();