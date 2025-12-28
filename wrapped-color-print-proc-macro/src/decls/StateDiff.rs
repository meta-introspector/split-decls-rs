macro_rules! deps {
    () => {
        ExtColor!();
        Action!();
    };
}

macro_rules! StateDiff {
    () => {
        deps!();
        # [doc = " The result of the comparison between two [`State`]s."] # [doc = ""] # [doc = " Each field is an [`Action`], which indicates if the given value has to be changed or left"] # [doc = " unchanged in order to reach the new state."] # [derive (Debug)] pub struct StateDiff { foreground : Action < ExtColor > , background : Action < ExtColor > , bold : Action < bool > , dim : Action < bool > , underline : Action < bool > , italics : Action < bool > , blink : Action < bool > , # [cfg (not (feature = "terminfo"))] strike : Action < bool > , reverse : Action < bool > , # [cfg (not (feature = "terminfo"))] conceal : Action < bool > , }
    };
}

StateDiff!()