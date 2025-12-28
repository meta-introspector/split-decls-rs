macro_rules! deps {
    () => {
        AllocatorTy!();
    };
}

macro_rules! AllocatorMethodInput {
    () => {
        deps!();
        pub struct AllocatorMethodInput { pub name : & 'static str , pub ty : AllocatorTy , }
    };
}

AllocatorMethodInput!()