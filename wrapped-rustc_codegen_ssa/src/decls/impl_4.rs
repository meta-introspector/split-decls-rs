macro_rules! deps {
    () => {
        CguReuse!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl fmt :: Display for CguReuse { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { CguReuse :: No => write ! (f , "No") , CguReuse :: PreLto => write ! (f , "PreLto") , CguReuse :: PostLto => write ! (f , "PostLto") , } } }
    };
}

impl_4!()