macro_rules! impl_808 {
    () => {
        impl < St1 , St2 , Clos , State > fmt :: Debug for SelectWithStrategy < St1 , St2 , Clos , State > where St1 : fmt :: Debug , St2 : fmt :: Debug , State : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SelectWithStrategy") . field ("stream1" , & self . stream1) . field ("stream2" , & self . stream2) . field ("state" , & self . state) . finish () } }
    };
}

impl_808!()