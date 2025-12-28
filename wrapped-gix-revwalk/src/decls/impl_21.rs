macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T > Clone for Commit < T > where T : Clone , { fn clone (& self) -> Self { Commit { parents : self . parents . clone () , commit_time : self . commit_time , generation : self . generation , data : self . data . clone () , } } }
    };
}

impl_21!();