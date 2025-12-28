macro_rules! deps {
    () => {
        ObjectDetached!();
        Kind!();
        Commit!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < 'repo > From < Commit < 'repo > > for ObjectDetached { fn from (mut v : Commit < 'repo >) -> Self { ObjectDetached { id : v . id , kind : gix_object :: Kind :: Commit , data : steal_from_freelist (& mut v . data) , } } }
    };
}

impl_163!();