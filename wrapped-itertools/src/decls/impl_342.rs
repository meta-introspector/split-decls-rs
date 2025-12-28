macro_rules! deps {
    () => {
        LazyBuffer!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < I , J > Index < J > for LazyBuffer < I > where I : Iterator , I :: Item : Sized , Vec < I :: Item > : Index < J > , { type Output = < Vec < I :: Item > as Index < J > > :: Output ; fn index (& self , index : J) -> & Self :: Output { self . buffer . index (index) } }
    };
}

impl_342!()