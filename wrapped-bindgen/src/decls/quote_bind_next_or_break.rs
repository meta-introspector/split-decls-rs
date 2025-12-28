macro_rules! deps {
    () => {
        RepInterp!();
    };
}

macro_rules! quote_bind_next_or_break {
    () => {
        deps!();
        # [macro_export] # [doc (hidden)] macro_rules ! quote_bind_next_or_break { ($ var : ident) => { let $ var = match $ var . next () { Some (_x) => $ crate :: tokens :: runtime :: RepInterp (_x) , None => break , } ; } ; }
    };
}

quote_bind_next_or_break!()