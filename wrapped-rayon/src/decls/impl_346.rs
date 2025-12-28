macro_rules! deps {
    () => {
        CollectResult!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl < 'c , T > Drop for CollectResult < 'c , T > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (ptr :: slice_from_raw_parts_mut (self . start . 0 , self . initialized_len ,)) ; } } }
    };
}

impl_346!()