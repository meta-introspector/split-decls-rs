macro_rules! deps {
    () => {
        ModPath!();
        Name!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl Extend < Name > for ModPath { fn extend < T : IntoIterator < Item = Name > > (& mut self , iter : T) { self . segments . extend (iter) ; } }
    };
}

impl_127!()