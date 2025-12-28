macro_rules! deps {
    () => {
        TransitProcess!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Clone for TransitProcess { fn clone (& self) -> TransitProcess { TransitProcess { copied_bytes : self . copied_bytes , total_bytes : self . total_bytes , file_bytes_copied : self . file_bytes_copied , file_total_bytes : self . file_total_bytes , file_name : self . file_name . clone () , state : self . state . clone () , } } }
    };
}

impl_33!();