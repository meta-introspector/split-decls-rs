macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl PartialEq for Commit < '_ > { fn eq (& self , other : & Self) -> bool { std :: ptr :: eq (self . file , other . file) && self . pos == other . pos } }
    };
}

impl_18!();