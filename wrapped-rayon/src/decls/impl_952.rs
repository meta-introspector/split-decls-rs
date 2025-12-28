macro_rules! deps {
    () => {
        UnzipConsumer!();
        Folder!();
        UnzipReducer!();
        Reducer!();
        UnzipOp!();
        UnzipFolder!();
        Consumer!();
    };
}

macro_rules! impl_952 {
    () => {
        deps!();
        impl < 'a , T , OP , CA , CB > Consumer < T > for UnzipConsumer < 'a , OP , CA , CB > where OP : UnzipOp < T > , CA : Consumer < OP :: Left > , CB : Consumer < OP :: Right > , { type Folder = UnzipFolder < 'a , OP , CA :: Folder , CB :: Folder > ; type Reducer = UnzipReducer < CA :: Reducer , CB :: Reducer > ; type Result = (CA :: Result , CB :: Result) ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left1 , left2 , left_reducer) = self . left . split_at (index) ; let (right1 , right2 , right_reducer) = self . right . split_at (index) ; (UnzipConsumer { op : self . op , left : left1 , right : right1 , } , UnzipConsumer { op : self . op , left : left2 , right : right2 , } , UnzipReducer { left : left_reducer , right : right_reducer , } ,) } fn into_folder (self) -> Self :: Folder { UnzipFolder { op : self . op , left : self . left . into_folder () , right : self . right . into_folder () , } } fn full (& self) -> bool { self . left . full () && self . right . full () } }
    };
}

impl_952!();