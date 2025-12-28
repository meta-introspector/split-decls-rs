macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl State { pub fn new (state : u32) -> Self { State { state } } pub fn update (& mut self , buf : & [u8]) { self . state = update_fast_16 (self . state , buf) ; } pub fn finalize (self) -> u32 { self . state } pub fn reset (& mut self) { self . state = 0 ; } pub fn combine (& mut self , other : u32 , amount : u64) { self . state = crate :: combine :: combine (self . state , other , amount) ; } }
    };
}

impl_1!();