macro_rules! deps {
    () => {
        Folder!();
        Consumer!();
        Reducer!();
        IntersperseConsumer!();
        IntersperseFolder!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl < C , T > Consumer < T > for IntersperseConsumer < C , T > where C : Consumer < T > , T : Clone + Send , { type Folder = IntersperseFolder < C :: Folder , T > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (mut self , index : usize) -> (Self , Self , Self :: Reducer) { let base_index = index + index . saturating_sub (! self . clone_first . get () as usize) ; let (left , right , reducer) = self . base . split_at (base_index) ; let right = IntersperseConsumer { base : right , item : self . item . clone () , clone_first : true . into () , } ; self . base = left ; (self , right , reducer) } fn into_folder (self) -> Self :: Folder { IntersperseFolder { base : self . base . into_folder () , item : self . item , clone_first : self . clone_first . get () , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_656!()