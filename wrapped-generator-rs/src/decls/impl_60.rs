macro_rules! deps {
    () => {
        StackBox!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < T > Drop for StackBox < T > { fn drop (& mut self) { let header = self . get_header () ; unsafe { * header . stack . get_offset () -= header . data_size + HEADER_SIZE ; ptr :: drop_in_place (self . ptr . as_ptr ()) ; if header . need_drop != 0 { header . stack . drop_stack () ; } } } }
    };
}

impl_60!();