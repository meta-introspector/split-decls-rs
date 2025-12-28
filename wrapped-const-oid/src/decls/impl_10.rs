macro_rules! deps {
    () => {
        Arc!();
        RootArcs!();
        Result!();
        Arcs!();
        Error!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a > Arcs < 'a > { # [doc = " Create a new iterator over an OID encoded as BER bytes."] pub (crate) fn new (bytes : & 'a [u8]) -> Self { Self { bytes , cursor : None , } } # [doc = " Try to parse the next arc in this OID."] # [doc = ""] # [doc = " This method is fallible so it can be used as a first pass to determine"] # [doc = " that the arcs in the OID are well-formed."] pub (crate) fn try_next (& mut self) -> Result < Option < Arc > > { match self . cursor { None => { let root_byte = * self . bytes . first () . ok_or (Error :: Empty) ? ; let root = RootArcs :: try_from (root_byte) ? ; self . cursor = Some (0) ; Ok (Some (root . first_arc ())) } Some (0) => { let root = RootArcs :: try_from (self . bytes [0]) ? ; self . cursor = Some (1) ; Ok (Some (root . second_arc ())) } Some (offset) => { let mut result = 0 ; let mut arc_bytes = 0 ; loop { let len = checked_add ! (offset , arc_bytes) ; match self . bytes . get (len) . cloned () { # [allow (clippy :: arithmetic_side_effects)] Some (byte) => { arc_bytes = checked_add ! (arc_bytes , 1) ; if (arc_bytes > ARC_MAX_BYTES) && (byte & ARC_MAX_LAST_OCTET != 0) { return Err (Error :: ArcTooBig) ; } result = (result << 7) | (byte & 0b1111111) as Arc ; if byte & 0b10000000 == 0 { self . cursor = Some (checked_add ! (offset , arc_bytes)) ; return Ok (Some (result)) ; } } None => { if arc_bytes == 0 { return Ok (None) ; } else { return Err (Error :: Base128) ; } } } } } } } }
    };
}

impl_10!();