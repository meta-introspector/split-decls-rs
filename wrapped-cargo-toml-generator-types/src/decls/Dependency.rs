macro_rules! deps {
    () => {
        DependencyTable!();
    };
}

macro_rules! Dependency {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq)] pub enum Dependency { Version (String) , Table (DependencyTable) , }
    };
}

Dependency!();