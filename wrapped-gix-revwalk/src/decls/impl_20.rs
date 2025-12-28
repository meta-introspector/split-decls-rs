macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T > std :: fmt :: Debug for Commit < T > where T : std :: fmt :: Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Commit") . field ("parents" , & self . parents) . field ("commit_time" , & self . commit_time) . field ("generation" , & self . generation) . field ("data" , & self . data) . finish () } }
    };
}

impl_20!()