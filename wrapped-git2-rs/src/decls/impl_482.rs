macro_rules! deps {
    () => {
        Object!();
        Error!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl < 'repo > std :: fmt :: Debug for Object < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("Object") ; match self . kind () { Some (kind) => ds . field ("kind" , & kind) , None => ds . field ("kind" , & format ! ("Unknow ({})" , unsafe { raw :: git_object_type (&* self . raw) }) ,) , } ; ds . field ("id" , & self . id ()) ; ds . finish () } }
    };
}

impl_482!();