macro_rules! deps {
    () => {
        AllocatorMethodInput!();
        AllocatorTy!();
    };
}

macro_rules! AllocatorMethod {
    () => {
        deps!();
        pub struct AllocatorMethod { pub name : Symbol , pub inputs : & 'static [AllocatorMethodInput] , pub output : AllocatorTy , }
    };
}

AllocatorMethod!()