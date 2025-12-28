macro_rules! NestedClass {
    () => {
        pub struct NestedClass { pub NestedClass : u32 , pub EnclosingClass : u32 , }
    };
}

NestedClass!();