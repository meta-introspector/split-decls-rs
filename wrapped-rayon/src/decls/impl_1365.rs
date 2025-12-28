macro_rules! deps {
    () => {
        SliceDrain!();
    };
}

macro_rules! impl_1365 {
    () => {
        deps!();
        impl < 'data , T : 'data > Drop for SliceDrain < 'data , T > { fn drop (& mut self) { let slice_ptr : * mut [T] = mem :: replace (& mut self . iter , [] . iter_mut ()) . into_slice () ; unsafe { ptr :: drop_in_place :: < [T] > (slice_ptr) } ; } }
    };
}

impl_1365!()