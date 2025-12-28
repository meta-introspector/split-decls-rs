macro_rules! deps {
    () => {
        MatchPosition!();
        FindReducer!();
        Reducer!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl < T > Reducer < Option < T > > for FindReducer { fn reduce (self , left : Option < T > , right : Option < T >) -> Option < T > { match self . match_position { MatchPosition :: Leftmost => left . or (right) , MatchPosition :: Rightmost => right . or (left) , } } }
    };
}

impl_506!()