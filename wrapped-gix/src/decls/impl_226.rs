macro_rules! deps {
    () => {
        Platform!();
        BreadthFirstPresets!();
        Error!();
        Entry!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl BreadthFirstPresets < '_ , '_ > { # [doc = " Returns all entries and their file paths, recursively, as reachable from this tree."] pub fn files (& self) -> Result < Vec < gix_traverse :: tree :: recorder :: Entry > , gix_traverse :: tree :: breadthfirst :: Error > { let mut recorder = gix_traverse :: tree :: Recorder :: default () ; Platform { root : self . root , breadthfirst : * self , } . breadthfirst (& mut recorder) ? ; Ok (recorder . records) } }
    };
}

impl_226!()