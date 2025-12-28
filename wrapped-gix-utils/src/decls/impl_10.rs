macro_rules! deps {
    () => {
        WithForeignSource!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl WithForeignSource < '_ , '_ > { # [doc = " Must be called after every change (i.e. when it's known that `dest` was written."] pub fn swap (& mut self) { self . ro_src . take () ; std :: mem :: swap (& mut self . src , & mut self . dest) ; self . dest . clear () ; } # [doc = " Obtain `(source, destination)`, which reads from the read-only source exactly once."] pub fn src_and_dest (& mut self) -> (& [u8] , & mut Vec < u8 >) { match self . ro_src { Some (src) => (src , & mut self . dest) , None => (self . src , & mut self . dest) , } } }
    };
}

impl_10!()