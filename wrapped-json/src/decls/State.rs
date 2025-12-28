macro_rules! State {
    () => {
        # [doc (hidden)] # [derive (Eq , PartialEq)] pub enum State { Empty , First , Rest , }
    };
}

State!()