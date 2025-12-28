macro_rules! deps {
    () => {
        CapturesMatches!();
        Captures!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl < 'r , 'c , 'h > Iterator for CapturesMatches < 'r , 'c , 'h > { type Item = Captures ; # [inline] fn next (& mut self) -> Option < Captures > { let CapturesMatches { re , ref mut cache , ref mut caps , ref mut it } = * self ; it . advance (| input | { re . search (cache , input , caps) ; Ok (caps . get_match ()) }) ; if caps . is_match () { Some (caps . clone ()) } else { None } } }
    };
}

impl_546!()