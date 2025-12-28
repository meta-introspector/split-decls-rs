macro_rules! NoFragmentCycles {
    () => {
        # [derive (Default)] pub struct NoFragmentCycles < 'a > { current_fragment : Option < & 'a str > , spreads : HashMap < & 'a str , Vec < (& 'a str , Pos) > > , fragment_order : Vec < & 'a str > , }
    };
}

NoFragmentCycles!()