macro_rules! Any {
    () => {
        pub struct Any { value : Value , drop : unsafe fn (& mut Value) , type_id : TypeId , # [doc = " For panic messages only. Not used for comparison."] # [cfg (feature = "unstable-debug")] type_name : & 'static str , }
    };
}

Any!();