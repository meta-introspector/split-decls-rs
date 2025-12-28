macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl Clone for Ty { fn clone (& self) -> Self { ensure_sufficient_stack (| | Self { id : self . id , kind : self . kind . clone () , span : self . span , tokens : self . tokens . clone () , }) } }
    };
}

impl_130!();