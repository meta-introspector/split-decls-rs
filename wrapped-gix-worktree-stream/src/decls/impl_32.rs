macro_rules! deps {
    () => {
        AdditionalEntry!();
        Stream!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Stream { pub (crate) fn new () -> (Stream , gix_features :: io :: pipe :: Writer , std :: sync :: mpsc :: Receiver < AdditionalEntry > ,) { let in_flight_writes = (2 + 1) * 32 ; let (write , read) = gix_features :: io :: pipe :: unidirectional (in_flight_writes) ; let (tx_entries , rx_entries) = std :: sync :: mpsc :: channel () ; (Stream { read : utils :: Read :: Known (read) , extra_entries : Some (tx_entries) , path_buf : Some (Vec :: with_capacity (1024) . into ()) , err : Default :: default () , buf : std :: iter :: repeat_n (0 , u16 :: MAX as usize) . collect () , pos : 0 , filled : 0 , } , write , rx_entries ,) } }
    };
}

impl_32!();