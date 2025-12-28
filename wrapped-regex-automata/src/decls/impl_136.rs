macro_rules! deps {
    () => {
        PatternID!();
        StartStateIter!();
        StateID!();
        Start!();
        Anchored!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < 'a , T : AsRef < [u8] > > Iterator for StartStateIter < 'a , T > { type Item = (StateID , Anchored , Start) ; fn next (& mut self) -> Option < (StateID , Anchored , Start) > { let i = self . i ; if i >= self . st . len () { return None ; } self . i += 1 ; let start_type = Start :: from_usize (i % self . st . stride) . unwrap () ; let anchored = if i < self . st . stride { Anchored :: No } else if i < (2 * self . st . stride) { Anchored :: Yes } else { let pid = (i - (2 * self . st . stride)) / self . st . stride ; Anchored :: Pattern (PatternID :: new (pid) . unwrap ()) } ; let start = i * StateID :: SIZE ; let end = start + StateID :: SIZE ; let bytes = self . st . table () [start .. end] . try_into () . unwrap () ; let id = StateID :: from_ne_bytes_unchecked (bytes) ; Some ((id , anchored , start_type)) } }
    };
}

impl_136!();