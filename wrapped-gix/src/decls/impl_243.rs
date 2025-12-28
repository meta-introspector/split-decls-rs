macro_rules! deps {
    () => {
        Commit!();
        ObjectDetached!();
        Kind!();
        Tree!();
        Blob!();
        Tag!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ObjectDetached { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { use gix_object :: Kind :: * ; let type_name = match self . kind { Blob => "Blob" , Commit => "Commit" , Tree => "Tree" , Tag => "Tag" , } ; write ! (f , "{}({})" , type_name , self . id) } }
    };
}

impl_243!()