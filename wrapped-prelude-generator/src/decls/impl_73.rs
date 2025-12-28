macro_rules! deps {
    () => {
        DependencyCollector!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl DependencyCollector { pub fn new () -> Self { DependencyCollector { dependencies : HashSet :: new () , } } fn add_dependency (& mut self , ident : & Ident) { self . dependencies . insert (ident . to_string ()) ; } }
    };
}

impl_73!()