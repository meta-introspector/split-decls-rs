macro_rules! deps {
    () => {
        ImageImportDescriptor!();
    };
}

macro_rules! impl_5338 {
    () => {
        deps!();
        impl ImageImportDescriptor { # [doc = " Tell whether this import descriptor is the null descriptor"] # [doc = " (used to mark the end of the iterator array in a PE)"] pub fn is_null (& self) -> bool { self . original_first_thunk . get (LE) == 0 && self . time_date_stamp . get (LE) == 0 && self . forwarder_chain . get (LE) == 0 && self . name . get (LE) == 0 && self . first_thunk . get (LE) == 0 } }
    };
}

impl_5338!();