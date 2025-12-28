macro_rules! deps {
    () => {
        PushPos!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < T > PushPos < T > for Vec < T > { fn push_pos (& mut self , value : T) -> u32 { self . push (value) ; (self . len () - 1) . try_into () . unwrap () } }
    };
}

impl_193!()