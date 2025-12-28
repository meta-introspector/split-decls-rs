macro_rules! deps {
    () => {
        Figure!();
        Size!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Set < Size > for Figure { # [doc = " Changes the figure size"] fn set (& mut self , size : Size) -> & mut Figure { self . size = Some ((size . 0 , size . 1)) ; self } }
    };
}

impl_21!()