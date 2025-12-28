macro_rules! deps {
    () => {
        Frame!();
        FrameNote!();
    };
}

macro_rules! get_span_and_frames {
    () => {
        deps!();
        pub fn get_span_and_frames < 'tcx > (tcx : TyCtxtAt < 'tcx > , stack : & [Frame < 'tcx , impl Provenance , impl Sized >] ,) -> (Span , Vec < errors :: FrameNote >) { let mut stacktrace = Frame :: generate_stacktrace_from_stack (stack) ; stacktrace . retain (| frame | ! frame . instance . def . requires_caller_location (* tcx)) ; let span = stacktrace . last () . map (| f | f . span) . unwrap_or (tcx . span) ; let mut frames = Vec :: new () ; if stacktrace . len () > 1 { let mut add_frame = | mut frame : errors :: FrameNote | { frames . push (errors :: FrameNote { times : 0 , .. frame . clone () }) ; if frame . times < 3 { let times = frame . times ; frame . times = 0 ; frames . extend (std :: iter :: repeat (frame) . take (times as usize)) ; } else { frames . push (frame) ; } } ; let mut last_frame : Option < errors :: FrameNote > = None ; for frame_info in & stacktrace { let frame = frame_info . as_note (* tcx) ; match last_frame . as_mut () { Some (last_frame) if last_frame . span == frame . span && last_frame . where_ == frame . where_ && last_frame . instance == frame . instance => { last_frame . times += 1 ; } Some (last_frame) => { add_frame (mem :: replace (last_frame , frame)) ; } None => { last_frame = Some (frame) ; } } } if let Some (frame) = last_frame { add_frame (frame) ; } } frames . reverse () ; if frames . len () > 0 { frames . remove (0) ; } if let Some (last) = frames . last_mut () && tcx . sess . source_map () . span_to_snippet (last . span . source_callsite ()) . is_ok () { last . has_label = true ; } (span , frames) }
    };
}

get_span_and_frames!()