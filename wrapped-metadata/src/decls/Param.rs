macro_rules! Param {
    () => {
        pub struct Param { pub Flags : ParamAttributes , pub Sequence : u16 , pub Name : id :: StringId , }
    };
}

Param!()