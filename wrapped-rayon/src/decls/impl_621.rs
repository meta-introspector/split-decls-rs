macro_rules! deps {
    () => {
        Consumer!();
        InspectConsumer!();
        Reducer!();
        InspectFolder!();
        Folder!();
    };
}

macro_rules! impl_621 {
    () => {
        deps!();
        impl < 'f , T , C , F > Consumer < T > for InspectConsumer < 'f , C , F > where C : Consumer < T > , F : Fn (& T) + Sync , { type Folder = InspectFolder < 'f , C :: Folder , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (InspectConsumer :: new (left , self . inspect_op) , InspectConsumer :: new (right , self . inspect_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { InspectFolder { base : self . base . into_folder () , inspect_op : self . inspect_op , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_621!();