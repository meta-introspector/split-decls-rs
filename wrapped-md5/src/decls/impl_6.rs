macro_rules! deps {
    () => {
        Md5Core!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl UpdateCore for Md5Core { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len = self . block_len . wrapping_add (blocks . len () as u64) ; let blocks = Array :: cast_slice_to_core (blocks) ; compress (& mut self . state , blocks) } }
    };
}

impl_6!()