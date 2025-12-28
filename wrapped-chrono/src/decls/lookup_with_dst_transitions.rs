macro_rules! deps {
    () => {
        FixedOffset!();
        Transition!();
        NaiveDateTime!();
        MappedLocalTime!();
    };
}

macro_rules! lookup_with_dst_transitions {
    () => {
        deps!();
        # [cfg (windows)] fn lookup_with_dst_transitions (transitions : & [Transition] , dt : NaiveDateTime ,) -> MappedLocalTime < FixedOffset > { for t in transitions . iter () { let (offset_min , offset_max) = match t . offset_after . local_minus_utc () > t . offset_before . local_minus_utc () { true => (t . offset_before , t . offset_after) , false => (t . offset_after , t . offset_before) , } ; let wall_earliest = t . transition_utc . overflowing_add_offset (offset_min) ; let wall_latest = t . transition_utc . overflowing_add_offset (offset_max) ; if dt < wall_earliest { return MappedLocalTime :: Single (t . offset_before) ; } else if dt <= wall_latest { return match t . offset_after . local_minus_utc () . cmp (& t . offset_before . local_minus_utc ()) { Ordering :: Equal => MappedLocalTime :: Single (t . offset_before) , Ordering :: Less => MappedLocalTime :: Ambiguous (t . offset_before , t . offset_after) , Ordering :: Greater => { if dt == wall_earliest { MappedLocalTime :: Single (t . offset_before) } else if dt == wall_latest { MappedLocalTime :: Single (t . offset_after) } else { MappedLocalTime :: None } } } ; } } MappedLocalTime :: Single (transitions . last () . unwrap () . offset_after) }
    };
}

lookup_with_dst_transitions!();