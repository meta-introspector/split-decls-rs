macro_rules! deps {
    () => {
        IntoIter!();
        WindowsProducer!();
        Windows!();
        Producer!();
    };
}

macro_rules! impl_1256 {
    () => {
        deps!();
        impl < 'data , T : 'data + Sync > Producer for WindowsProducer < 'data , T > { type Item = & 'data [T] ; type IntoIter = :: std :: slice :: Windows < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . windows (self . window_size) } fn split_at (self , index : usize) -> (Self , Self) { let left_index = Ord :: min (self . slice . len () , index + (self . window_size - 1)) ; let left = & self . slice [.. left_index] ; let right = & self . slice [index ..] ; (WindowsProducer { window_size : self . window_size , slice : left , } , WindowsProducer { window_size : self . window_size , slice : right , } ,) } }
    };
}

impl_1256!()