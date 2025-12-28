macro_rules! deps {
    () => {
        InternalState!();
        PollNext!();
        Ready!();
    };
}

macro_rules! impl_807 {
    () => {
        deps!();
        impl < St1 , St2 , Clos , State > Stream for SelectWithStrategy < St1 , St2 , Clos , State > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , Clos : FnMut (& mut State) -> PollNext , { type Item = St1 :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St1 :: Item > > { let mut this = self . project () ; match this . internal_state { InternalState :: Start => { let next_side = (this . clos) (this . state) ; poll_inner (& mut this , next_side , cx) } InternalState :: LeftFinished => match this . stream2 . poll_next (cx) { Poll :: Ready (None) => { * this . internal_state = InternalState :: BothFinished ; Poll :: Ready (None) } a => a , } , InternalState :: RightFinished => match this . stream1 . poll_next (cx) { Poll :: Ready (None) => { * this . internal_state = InternalState :: BothFinished ; Poll :: Ready (None) } a => a , } , InternalState :: BothFinished => Poll :: Ready (None) , } } }
    };
}

impl_807!();