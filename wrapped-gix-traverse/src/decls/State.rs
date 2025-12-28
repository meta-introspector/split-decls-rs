macro_rules! State {
    () => {
        # [doc = " The state used and potentially shared by multiple tree traversals, reusing memory."] # [derive (Default , Clone)] pub struct State { freelist : Vec < Vec < u8 > > , }
    };
}

State!()