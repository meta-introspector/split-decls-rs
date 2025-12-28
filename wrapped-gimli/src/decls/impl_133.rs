macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        # [cfg (feature = "read")] impl < T > ArrayVec < Vec < T > > { pub fn into_vec (mut self) -> Vec < T > { let len = core :: mem :: replace (& mut self . len , 0) ; let storage = core :: mem :: replace (& mut self . storage , Box :: new ([])) ; let slice = Box :: leak (storage) ; debug_assert ! (len <= slice . len ()) ; unsafe { Vec :: from_raw_parts (slice . as_mut_ptr () as * mut T , len , slice . len ()) } } }
    };
}

impl_133!();