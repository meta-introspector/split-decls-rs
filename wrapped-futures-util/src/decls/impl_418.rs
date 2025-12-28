macro_rules! impl_418 {
    () => {
        impl < St , T > fmt :: Debug for NextIfEq < '_ , St , T > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , T : ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("NextIfEq") . field ("inner" , & self . inner . inner . as_ref () . map (| (s , _f) | s)) . finish () } }
    };
}

impl_418!()