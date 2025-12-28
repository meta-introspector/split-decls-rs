macro_rules! deps {
    () => {
        ImageDelayloadDescriptor!();
    };
}

macro_rules! impl_5342 {
    () => {
        deps!();
        impl ImageDelayloadDescriptor { # [doc = " Tell whether this delay-load import descriptor is the null descriptor"] # [doc = " (used to mark the end of the iterator array in a PE)"] pub fn is_null (& self) -> bool { self . attributes . get (LE) == 0 && self . dll_name_rva . get (LE) == 0 && self . module_handle_rva . get (LE) == 0 && self . import_address_table_rva . get (LE) == 0 && self . import_name_table_rva . get (LE) == 0 && self . bound_import_address_table_rva . get (LE) == 0 && self . unload_information_table_rva . get (LE) == 0 && self . time_date_stamp . get (LE) == 0 } }
    };
}

impl_5342!();