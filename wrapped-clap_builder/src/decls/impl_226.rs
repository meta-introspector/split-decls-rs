macro_rules! impl_226 {
    () => {
        impl PartialOrd for Inner { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { Some (self . cmp (other)) } }
    };
}

impl_226!()