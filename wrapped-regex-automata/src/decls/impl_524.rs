macro_rules! deps {
    () => {
        StateID!();
        DenseTransitions!();
        Unit!();
        Usize!();
        Transition!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl DenseTransitions { # [doc = " This follows the matching transition for a particular byte."] # [doc = ""] # [doc = " The matching transition is found by looking for a transition that"] # [doc = " doesn't correspond to `StateID::ZERO` for the byte `at` the given"] # [doc = " position in `haystack`."] # [doc = ""] # [doc = " If `at >= haystack.len()`, then this returns `None`."] # [inline] pub fn matches (& self , haystack : & [u8] , at : usize) -> Option < StateID > { haystack . get (at) . and_then (| & b | self . matches_byte (b)) } # [doc = " This follows the matching transition for any member of the alphabet."] # [doc = ""] # [doc = " The matching transition is found by looking for a transition that"] # [doc = " doesn't correspond to `StateID::ZERO` for the given alphabet `unit`."] # [doc = ""] # [doc = " If the given alphabet unit is [`EOI`](alphabet::Unit::eoi), then"] # [doc = " this returns `None`."] # [inline] pub (crate) fn matches_unit (& self , unit : alphabet :: Unit ,) -> Option < StateID > { unit . as_u8 () . and_then (| byte | self . matches_byte (byte)) } # [doc = " This follows the matching transition for a particular byte."] # [doc = ""] # [doc = " The matching transition is found by looking for a transition that"] # [doc = " doesn't correspond to `StateID::ZERO` for the given `byte`."] # [inline] pub fn matches_byte (& self , byte : u8) -> Option < StateID > { let next = self . transitions [usize :: from (byte)] ; if next == StateID :: ZERO { None } else { Some (next) } } # [doc = " Returns an iterator over all transitions that don't point to"] # [doc = " `StateID::ZERO`."] pub (crate) fn iter (& self) -> impl Iterator < Item = Transition > + '_ { use crate :: util :: int :: Usize ; self . transitions . iter () . enumerate () . filter (| & (_ , & sid) | sid != StateID :: ZERO) . map (| (byte , & next) | Transition { start : byte . as_u8 () , end : byte . as_u8 () , next , }) } }
    };
}

impl_524!();