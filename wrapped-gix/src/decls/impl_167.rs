macro_rules! deps {
    () => {
        Commit!();
        Object!();
        Kind!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'repo > From < Commit < 'repo > > for Object < 'repo > { fn from (mut v : Commit < 'repo >) -> Self { Object { id : v . id , kind : gix_object :: Kind :: Commit , data : steal_from_freelist (& mut v . data) , repo : v . repo , } } }
    };
}

impl_167!()