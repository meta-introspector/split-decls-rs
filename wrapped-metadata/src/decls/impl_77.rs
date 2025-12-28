macro_rules! deps {
    () => {
        AssemblyRef!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl std :: fmt :: Debug for AssemblyRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { f . debug_tuple ("AssemblyRef") . field (& self . 0) . finish () } }
    };
}

impl_77!()