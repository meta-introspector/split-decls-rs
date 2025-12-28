macro_rules! deps {
    () => {
        Object!();
        Kind!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T : cache :: Object + ? Sized > cache :: Object for Box < T > { fn put (& mut self , id : gix_hash :: ObjectId , kind : gix_object :: Kind , data : & [u8]) { use std :: ops :: DerefMut ; self . deref_mut () . put (id , kind , data) ; } fn get (& mut self , id : & gix_hash :: ObjectId , out : & mut Vec < u8 >) -> Option < gix_object :: Kind > { use std :: ops :: DerefMut ; self . deref_mut () . get (id , out) } }
    };
}

impl_43!()