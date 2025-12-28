macro_rules! deps {
    () => {
        InlinedFunction!();
        Frame!();
        FrameIter!();
        Function!();
        FrameIterFrames!();
        Result!();
        FrameIterState!();
        FunctionName!();
        Error!();
        Location!();
        ResUnit!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'ctx , R > FrameIter < 'ctx , R > where R : gimli :: Reader + 'ctx , { pub (crate) fn new_empty () -> Self { FrameIter (FrameIterState :: Empty) } pub (crate) fn new_location (location : Location < 'ctx >) -> Self { FrameIter (FrameIterState :: Location (Some (location))) } pub (crate) fn new_frames (unit : & 'ctx ResUnit < R > , sections : & 'ctx gimli :: Dwarf < R > , function : & 'ctx Function < R > , inlined_functions : maybe_small :: Vec < & 'ctx InlinedFunction < R > > , location : Option < Location < 'ctx > > ,) -> Self { FrameIter (FrameIterState :: Frames (FrameIterFrames { unit , sections , function , inlined_functions : inlined_functions . into_iter () . rev () , next : location , })) } # [doc = " Advances the iterator and returns the next frame."] # [allow (clippy :: should_implement_trait)] pub fn next (& mut self) -> Result < Option < Frame < 'ctx , R > > , Error > { let frames = match & mut self . 0 { FrameIterState :: Empty => return Ok (None) , FrameIterState :: Location (location) => { let location = location . take () ; self . 0 = FrameIterState :: Empty ; return Ok (Some (Frame { dw_die_offset : None , function : None , location , })) ; } FrameIterState :: Frames (frames) => frames , } ; let loc = frames . next . take () ; let func = match frames . inlined_functions . next () { Some (func) => func , None => { let frame = Frame { dw_die_offset : Some (frames . function . dw_die_offset) , function : frames . function . name . clone () . map (| name | FunctionName { name , language : frames . unit . lang , }) , location : loc , } ; self . 0 = FrameIterState :: Empty ; return Ok (Some (frame)) ; } } ; let mut next = Location { file : None , line : if func . call_line != 0 { Some (func . call_line) } else { None } , column : if func . call_column != 0 { Some (func . call_column) } else { None } , } ; if let Some (call_file) = func . call_file { if let Some (lines) = frames . unit . parse_lines (frames . sections) ? { next . file = lines . file (call_file) ; } } frames . next = Some (next) ; Ok (Some (Frame { dw_die_offset : Some (func . dw_die_offset) , function : func . name . clone () . map (| name | FunctionName { name , language : frames . unit . lang , }) , location : loc , })) } }
    };
}

impl_12!()