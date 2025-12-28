macro_rules! deps {
    () => {
        DrainProducer!();
    };
}

macro_rules! impl_1359 {
    () => {
        deps!();
        impl < 'data , T : 'data + Send > Drop for DrainProducer < 'data , T > { fn drop (& mut self) { let slice_ptr : * mut [T] = mem :: take :: < & 'data mut [T] > (& mut self . slice) ; unsafe { ptr :: drop_in_place :: < [T] > (slice_ptr) } ; } }
    };
}

impl_1359!()