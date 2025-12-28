macro_rules! PushPos {
    () => {
        pub trait PushPos < T > { fn push_pos (& mut self , value : T) -> u32 ; }
    };
}

PushPos!()