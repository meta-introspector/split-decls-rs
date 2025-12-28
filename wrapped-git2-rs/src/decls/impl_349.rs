macro_rules! deps {
    () => {
        DiffHunk!();
        Error!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < 'a > std :: fmt :: Debug for DiffHunk < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("DiffHunk") . field ("old_start" , & self . old_start ()) . field ("old_lines" , & self . old_lines ()) . field ("new_start" , & self . new_start ()) . field ("new_lines" , & self . new_lines ()) . field ("header" , & self . header ()) . finish () } }
    };
}

impl_349!();