macro_rules! deps {
    () => {
        TaggedArcPtr!();
    };
}

macro_rules! Symbol {
    () => {
        deps!();
        # [derive (PartialEq , Eq , Hash)] pub struct Symbol { repr : TaggedArcPtr , }
    };
}

Symbol!();