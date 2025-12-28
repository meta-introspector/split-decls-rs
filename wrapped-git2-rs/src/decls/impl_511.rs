macro_rules! deps {
    () => {
        OdbPackwriter!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl < 'repo > Drop for OdbPackwriter < 'repo > { fn drop (& mut self) { unsafe { let writepack = & * self . raw ; match writepack . free { Some (free) => free (self . raw) , None => () , } ; drop (Box :: from_raw (self . progress_payload_ptr)) ; } } }
    };
}

impl_511!();