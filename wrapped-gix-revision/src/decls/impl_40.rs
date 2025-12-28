macro_rules! deps {
    () => {
        SiblingBranch!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl SiblingBranch { # [doc = " Parse `input` as branch representation, if possible."] pub fn parse (input : & BStr) -> Option < Self > { if input . eq_ignore_ascii_case (b"u") || input . eq_ignore_ascii_case (b"upstream") { SiblingBranch :: Upstream . into () } else if input . eq_ignore_ascii_case (b"push") { SiblingBranch :: Push . into () } else { None } } }
    };
}

impl_40!();