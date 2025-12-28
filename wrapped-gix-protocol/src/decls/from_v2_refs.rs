macro_rules! deps {
    () => {
        Ref!();
        Error!();
    };
}

macro_rules! from_v2_refs {
    () => {
        deps!();
        # [doc = " Parse refs from the given input line by line. Protocol V2 is required for this to succeed."] pub fn from_v2_refs (in_refs : & mut dyn ReadlineBufRead) -> Result < Vec < Ref > , Error > { let mut out_refs = Vec :: new () ; while let Some (line) = in_refs . readline () . transpose () ? . transpose () ? . and_then (| l | l . as_bstr ()) { out_refs . push (refs :: shared :: parse_v2 (line) ?) ; } Ok (out_refs) }
    };
}

from_v2_refs!();