macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl PartialEq for File { fn eq (& self , other : & Self) -> bool { std :: ptr :: eq (self . bytes . as_ptr () , other . bytes . as_ptr ()) } }
    };
}

impl_436!()