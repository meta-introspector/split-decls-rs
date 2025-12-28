macro_rules! deps {
    () => {
        Acknowledgement!();
        WantedRef!();
    };
}

macro_rules! Response {
    () => {
        deps!();
        # [doc = " A representation of a complete fetch response"] # [derive (Debug , Clone)] pub struct Response { pub (crate) acks : Vec < Acknowledgement > , pub (crate) shallows : Vec < ShallowUpdate > , pub (crate) wanted_refs : Vec < WantedRef > , pub (crate) has_pack : bool , }
    };
}

Response!()