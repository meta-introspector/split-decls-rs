macro_rules! deps {
    () => {
        Crate!();
    };
}

macro_rules! CrateDependency {
    () => {
        deps!();
        # [derive (Debug)] pub struct CrateDependency { pub krate : Crate , pub name : Name , }
    };
}

CrateDependency!()