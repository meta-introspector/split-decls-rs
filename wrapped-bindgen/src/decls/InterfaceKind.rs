macro_rules! InterfaceKind {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] pub enum InterfaceKind { None , Default , Static , Composable , Base , }
    };
}

InterfaceKind!();