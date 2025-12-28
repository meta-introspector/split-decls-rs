macro_rules! deps {
    () => {
        ProgressState!();
        Convert!();
        Progress!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        impl < 'a > Progress < 'a > { # [doc = " Number of objects in the packfile being downloaded"] pub fn total_objects (& self) -> usize { unsafe { (* self . raw ()) . total_objects as usize } } # [doc = " Received objects that have been hashed"] pub fn indexed_objects (& self) -> usize { unsafe { (* self . raw ()) . indexed_objects as usize } } # [doc = " Objects which have been downloaded"] pub fn received_objects (& self) -> usize { unsafe { (* self . raw ()) . received_objects as usize } } # [doc = " Locally-available objects that have been injected in order to fix a thin"] # [doc = " pack."] pub fn local_objects (& self) -> usize { unsafe { (* self . raw ()) . local_objects as usize } } # [doc = " Number of deltas in the packfile being downloaded"] pub fn total_deltas (& self) -> usize { unsafe { (* self . raw ()) . total_deltas as usize } } # [doc = " Received deltas that have been hashed."] pub fn indexed_deltas (& self) -> usize { unsafe { (* self . raw ()) . indexed_deltas as usize } } # [doc = " Size of the packfile received up to now"] pub fn received_bytes (& self) -> usize { unsafe { (* self . raw ()) . received_bytes as usize } } # [doc = " Convert this to an owned version of `Progress`."] pub fn to_owned (& self) -> Progress < 'static > { Progress { raw : ProgressState :: Owned (unsafe { * self . raw () }) , _marker : marker :: PhantomData , } } }
    };
}

impl_401!()