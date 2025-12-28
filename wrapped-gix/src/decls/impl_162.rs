macro_rules! deps {
    () => {
        ObjectDetached!();
        Object!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < 'repo > From < Object < 'repo > > for ObjectDetached { fn from (mut v : Object < 'repo >) -> Self { ObjectDetached { id : v . id , kind : v . kind , data : steal_from_freelist (& mut v . data) , } } }
    };
}

impl_162!()