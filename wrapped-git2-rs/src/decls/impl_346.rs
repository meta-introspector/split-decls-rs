macro_rules! deps {
    () => {
        Error!();
        DiffLine!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl < 'a > std :: fmt :: Debug for DiffLine < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("DiffLine") ; if let Some (old_lineno) = & self . old_lineno () { ds . field ("old_lineno" , old_lineno) ; } if let Some (new_lineno) = & self . new_lineno () { ds . field ("new_lineno" , new_lineno) ; } ds . field ("num_lines" , & self . num_lines ()) . field ("content_offset" , & self . content_offset ()) . field ("content" , & self . content ()) . field ("origin" , & self . origin ()) . finish () } }
    };
}

impl_346!();