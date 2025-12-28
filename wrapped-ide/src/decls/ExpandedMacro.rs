macro_rules! ExpandedMacro {
    () => {
        pub struct ExpandedMacro { pub name : String , pub expansion : String , }
    };
}

ExpandedMacro!()