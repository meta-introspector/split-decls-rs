macro_rules! deps {
    () => {
        Instruction!();
        Push!();
        Fetch!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Instruction < '_ > { # [doc = " Reproduce ourselves in parseable form."] pub fn to_bstring (& self) -> BString { let mut buf = Vec :: with_capacity (128) ; self . write_to (& mut buf) . expect ("no io error") ; buf . into () } # [doc = " Serialize ourselves in a parseable format to `out`."] pub fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { match self { Instruction :: Push (Push :: Matching { src , dst , allow_non_fast_forward , }) => { if * allow_non_fast_forward { out . write_all (b"+") ? ; } out . write_all (src) ? ; out . write_all (b":") ? ; out . write_all (dst) } Instruction :: Push (Push :: AllMatchingBranches { allow_non_fast_forward }) => { if * allow_non_fast_forward { out . write_all (b"+") ? ; } out . write_all (b":") } Instruction :: Push (Push :: Delete { ref_or_pattern }) => { out . write_all (b":") ? ; out . write_all (ref_or_pattern) } Instruction :: Fetch (Fetch :: Only { src }) => out . write_all (src) , Instruction :: Fetch (Fetch :: Exclude { src }) | Instruction :: Push (Push :: Exclude { src }) => { out . write_all (b"^") ? ; out . write_all (src) } Instruction :: Fetch (Fetch :: AndUpdate { src , dst , allow_non_fast_forward , }) => { if * allow_non_fast_forward { out . write_all (b"+") ? ; } out . write_all (src) ? ; out . write_all (b":") ? ; out . write_all (dst) } } } }
    };
}

impl_18!()