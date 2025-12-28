macro_rules! impl_410 {
    () => {
        impl < St > fmt :: Debug for PeekMut < '_ , St > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PeekMut") . field ("inner" , & self . inner) . finish () } }
    };
}

impl_410!()