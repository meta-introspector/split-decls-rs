macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < K , V , S > Drop for LinkedHashMap < K , V , S > { # [inline] fn drop (& mut self) { unsafe { if let Some (values) = self . values { drop_value_nodes (values) ; let _ = Box :: from_raw (values . as_ptr ()) ; } drop_free_nodes (self . free) ; } } }
    };
}

impl_15!()