macro_rules! deps {
    () => {
        TryReduceWithFolder!();
        Folder!();
    };
}

macro_rules! impl_932 {
    () => {
        deps!();
        impl < 'r , R , T > Folder < T > for TryReduceWithFolder < 'r , R , T > where R : Fn (T :: Output , T :: Output) -> T , T : Try , { type Result = Option < T > ; fn consume (mut self , item : T) -> Self { let reduce_op = self . reduce_op ; let control = match (self . opt_control , item . branch ()) { (Some (Continue (left)) , Continue (right)) => reduce_op (left , right) . branch () , (Some (control @ Break (_)) , _) | (_ , control) => control , } ; if let Break (_) = control { self . full . store (true , Ordering :: Relaxed) } self . opt_control = Some (control) ; self } fn complete (self) -> Option < T > { match self . opt_control { Some (Continue (c)) => Some (T :: from_output (c)) , Some (Break (r)) => Some (T :: from_residual (r)) , None => None , } } fn full (& self) -> bool { match self . opt_control { Some (Break (_)) => true , _ => self . full . load (Ordering :: Relaxed) , } } }
    };
}

impl_932!()