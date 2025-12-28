macro_rules! deps {
    () => {
        Blob!();
        Tree!();
        Tag!();
        Commit!();
        Object!();
        Kind!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Object < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { use gix_object :: Kind :: * ; let type_name = match self . kind { Blob => "Blob" , Commit => "Commit" , Tree => "Tree" , Tag => "Tag" , } ; write ! (f , "{}({})" , type_name , self . id) } }
    };
}

impl_174!()