macro_rules! deps {
    () => {
        PanicFuseConsumer!();
        Folder!();
        Consumer!();
        PanicFuseFolder!();
        PanicFuseReducer!();
        Reducer!();
    };
}

macro_rules! impl_750 {
    () => {
        deps!();
        impl < 'a , T , C > Consumer < T > for PanicFuseConsumer < 'a , C > where C : Consumer < T > , { type Folder = PanicFuseFolder < 'a , C :: Folder > ; type Reducer = PanicFuseReducer < 'a , C :: Reducer > ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (PanicFuseConsumer { base : left , fuse : self . fuse . clone () , } , PanicFuseConsumer { base : right , fuse : self . fuse . clone () , } , PanicFuseReducer { base : reducer , _fuse : self . fuse , } ,) } fn into_folder (self) -> Self :: Folder { PanicFuseFolder { base : self . base . into_folder () , fuse : self . fuse , } } fn full (& self) -> bool { self . fuse . panicked () || self . base . full () } }
    };
}

impl_750!();