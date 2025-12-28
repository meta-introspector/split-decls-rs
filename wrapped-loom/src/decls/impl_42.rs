macro_rules! deps {
    () => {
        VersionVec!();
        Synchronize!();
        Store!();
        FirstSeen!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Default for Store { fn default () -> Store { Store { value : 0 , happens_before : VersionVec :: new () , modification_order : VersionVec :: new () , sync : Synchronize :: new () , first_seen : FirstSeen :: new () , seq_cst : false , } } }
    };
}

impl_42!();