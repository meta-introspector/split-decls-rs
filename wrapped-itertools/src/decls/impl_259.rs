macro_rules! deps {
    () => {
        GroupInner!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl < K , I , F > GroupInner < K , I , F > where I : Iterator , { # [doc = " Called when a group is dropped"] fn drop_group (& mut self , client : usize) { if self . dropped_group == ! 0 || client > self . dropped_group { self . dropped_group = client ; } } }
    };
}

impl_259!()