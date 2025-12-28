macro_rules! deps {
    () => {
        StateDiff!();
        ExtColor!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " Describes the state of each color and style attributes at a given position in the format"] # [doc = " string. Two states can be compared together by creating a [`StateDiff`] instance."] # [derive (Debug , PartialEq , Default)] pub struct State { foreground : ExtColor , background : ExtColor , bold : bool , dim : bool , underline : bool , italics : bool , blink : bool , strike : bool , reverse : bool , conceal : bool , }
    };
}

State!();