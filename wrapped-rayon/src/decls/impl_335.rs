macro_rules! deps {
    () => {
        Reducer!();
        Consumer!();
        ClonedFolder!();
        Folder!();
        ClonedConsumer!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < 'a , T , C > Consumer < & 'a T > for ClonedConsumer < C > where C : Consumer < T > , T : 'a + Clone , { type Folder = ClonedFolder < C :: Folder > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (ClonedConsumer :: new (left) , ClonedConsumer :: new (right) , reducer ,) } fn into_folder (self) -> Self :: Folder { ClonedFolder { base : self . base . into_folder () , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_335!()