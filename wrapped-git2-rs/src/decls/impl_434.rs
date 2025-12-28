macro_rules! deps {
    () => {
        MergeFileResult!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl std :: fmt :: Debug for MergeFileResult { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut ds = f . debug_struct ("MergeFileResult") ; if let Some (path) = & self . path () { ds . field ("path" , path) ; } ds . field ("automergeable" , & self . is_automergeable ()) ; ds . field ("mode" , & self . mode ()) ; ds . finish () } }
    };
}

impl_434!()