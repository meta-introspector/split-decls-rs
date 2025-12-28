macro_rules! deps {
    () => {
        BlobWriter!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < 'repo > Drop for BlobWriter < 'repo > { fn drop (& mut self) { if self . need_cleanup { unsafe { if let Some (f) = (* self . raw) . free { f (self . raw) } } } } }
    };
}

impl_224!()