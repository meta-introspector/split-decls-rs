macro_rules! deps {
    () => {
        Reducer!();
        Consumer!();
        FindConsumer!();
        FindReducer!();
        Folder!();
        FindFolder!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl < 'p , T , P > Consumer < T > for FindConsumer < 'p , P > where T : Send , P : Fn (& T) -> bool + Sync , { type Folder = FindFolder < 'p , T , P > ; type Reducer = FindReducer ; type Result = Option < T > ; fn split_at (self , _index : usize) -> (Self , Self , Self :: Reducer) { let dir = self . match_position ; (self . split_off_left () , self , FindReducer { match_position : dir , } ,) } fn into_folder (self) -> Self :: Folder { FindFolder { find_op : self . find_op , boundary : self . current_index () , match_position : self . match_position , best_found : self . best_found , item : None , } } fn full (& self) -> bool { better_position (self . best_found . load (Ordering :: Relaxed) , self . current_index () , self . match_position ,) } }
    };
}

impl_501!()