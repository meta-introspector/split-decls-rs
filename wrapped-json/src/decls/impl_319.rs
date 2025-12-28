macro_rules! deps {
    () => {
        Index!();
        Value!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < T > Index for & T where T : ? Sized + Index , { fn index_into < 'v > (& self , v : & 'v Value) -> Option < & 'v Value > { (* * self) . index_into (v) } fn index_into_mut < 'v > (& self , v : & 'v mut Value) -> Option < & 'v mut Value > { (* * self) . index_into_mut (v) } fn index_or_insert < 'v > (& self , v : & 'v mut Value) -> & 'v mut Value { (* * self) . index_or_insert (v) } }
    };
}

impl_319!();