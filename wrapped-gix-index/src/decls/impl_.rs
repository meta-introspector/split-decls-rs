macro_rules! deps {
    () => {
        State!();
        File!();
    };
}

macro_rules! impl_ {
    () => {
        deps!();
        mod impl_ { use std :: fmt :: Formatter ; use crate :: { File , State } ; impl std :: fmt :: Debug for File { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("File") . field ("path" , & self . path . display ()) . field ("checksum" , & self . checksum) . finish_non_exhaustive () } } impl From < File > for State { fn from (f : File) -> Self { f . state } } }
    };
}

impl_!();