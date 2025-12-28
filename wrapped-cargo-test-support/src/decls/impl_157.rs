macro_rules! deps {
    () => {
        Execs!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl Execs { pub fn with_process_builder (mut self , p : ProcessBuilder) -> Execs { self . process_builder = Some (p) ; self } }
    };
}

impl_157!();