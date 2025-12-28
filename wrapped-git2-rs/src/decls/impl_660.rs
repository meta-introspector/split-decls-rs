macro_rules! deps {
    () => {
        RemoteConnection!();
    };
}

macro_rules! impl_660 {
    () => {
        deps!();
        impl < 'repo , 'connection , 'cb > Drop for RemoteConnection < 'repo , 'connection , 'cb > { fn drop (& mut self) { drop (self . remote . disconnect ()) ; } }
    };
}

impl_660!();