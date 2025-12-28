macro_rules! deps {
    () => {
        ModPath!();
        Name!();
        PathKind!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl From < Name > for ModPath { fn from (name : Name) -> ModPath { ModPath :: from_segments (PathKind :: Plain , iter :: once (name)) } }
    };
}

impl_130!();