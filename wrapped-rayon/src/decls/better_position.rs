macro_rules! deps {
    () => {
        MatchPosition!();
    };
}

macro_rules! better_position {
    () => {
        deps!();
        # [doc = " Returns true if pos1 is a better match than pos2 according to MatchPosition"] # [inline] fn better_position (pos1 : usize , pos2 : usize , mp : MatchPosition) -> bool { match mp { MatchPosition :: Leftmost => pos1 < pos2 , MatchPosition :: Rightmost => pos1 > pos2 , } }
    };
}

better_position!();