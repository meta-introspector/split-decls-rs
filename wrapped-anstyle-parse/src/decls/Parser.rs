macro_rules! deps {
    () => {
        Perform!();
        DefaultCharAccumulator!();
    };
}

macro_rules! Parser {
    () => {
        deps!();
        # [doc = " Parser for raw _VTE_ protocol which delegates actions to a [`Perform`]"] # [allow (unused_qualifications)] # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct Parser < C = DefaultCharAccumulator > { state : State , intermediates : [u8 ; MAX_INTERMEDIATES] , intermediate_idx : usize , params : Params , param : u16 , # [cfg (feature = "core")] osc_raw : ArrayVec < u8 , MAX_OSC_RAW > , # [cfg (not (feature = "core"))] osc_raw : alloc :: vec :: Vec < u8 > , osc_params : [(usize , usize) ; MAX_OSC_PARAMS] , osc_num_params : usize , ignoring : bool , utf8_parser : C , }
    };
}

Parser!()