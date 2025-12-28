macro_rules! deps {
    () => {
        Options!();
        Level!();
        State!();
    };
}

macro_rules! all {
    () => {
        deps!();
        pub fn all (out : & mut impl io :: Write , show_progress : bool , state : & mut State , config : & Options) -> io :: Result < () > { if ! config . keep_running_if_progress_is_empty && state . tree . is_empty () { return Err (io :: Error :: new (io :: ErrorKind :: Other , "stop as progress is empty")) ; } messages (out , state , config . colored , config . terminal_dimensions . 1 as usize , config . timestamp ,) ? ; if show_progress && config . output_is_terminal { if let Some (tp) = state . throughput . as_mut () { tp . update_elapsed () ; } let level_range = config . level_filter . clone () . unwrap_or (RangeInclusive :: new (0 , progress :: key :: Level :: MAX)) ; let lines_to_be_drawn = state . tree . iter () . filter (| (k , _) | level_range . contains (& k . level ())) . count () ; if state . blocks_per_line . len () < lines_to_be_drawn { state . blocks_per_line . resize (lines_to_be_drawn , 0) ; } let mut tokens : Vec < ANSIString < '_ > > = Vec :: with_capacity (4) ; let mut max_midpoint = 0 ; for ((key , value) , ref mut blocks_in_last_iteration) in state . tree . iter () . filter (| (k , _) | level_range . contains (& k . level ())) . zip (state . blocks_per_line . iter_mut ()) { max_midpoint = max_midpoint . max (format_progress (key , value , config . terminal_dimensions . 0 , config . colored , state . last_progress_midpoint , state . throughput . as_mut () . and_then (| tp | tp . update_and_get (key , value . progress . as_ref ())) , & mut tokens ,) . unwrap_or (0) ,) ; write ! (out , "{}" , ANSIStrings (tokens . as_slice ())) ? ; * * blocks_in_last_iteration = newline_with_overdraw (out , & tokens , * * blocks_in_last_iteration) ? ; } if let Some (tp) = state . throughput . as_mut () { tp . reconcile (& state . tree) ; } state . last_progress_midpoint = Some (max_midpoint) ; let lines_drawn = lines_to_be_drawn ; if state . blocks_per_line . len () > lines_drawn { for blocks_in_last_iteration in state . blocks_per_line . iter () . skip (lines_drawn) { writeln ! (out , "{:>width$}" , "" , width = * blocks_in_last_iteration as usize) ? ; } crosstermion :: execute ! (out , crosstermion :: cursor :: MoveUp (state . blocks_per_line . len () as u16)) ? ; state . blocks_per_line . resize (lines_drawn , 0) ; } else if lines_drawn > 0 { crosstermion :: execute ! (out , crosstermion :: cursor :: MoveUp (lines_drawn as u16)) ? ; } } Ok (()) }
    };
}

all!();