macro_rules! deps {
    () => {
        Stage!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl Flags { # [doc = " Create a new instance whose stage is set to `stage`."] pub fn from_stage (stage : Stage) -> Self { Flags :: from_bits ((stage as u32) << 12) . expect ("stage can only be valid flags") } # [doc = " Return the stage as extracted from the bits of this instance."] pub fn stage (& self) -> Stage { match self . stage_raw () { 0 => Stage :: Unconflicted , 1 => Stage :: Base , 2 => Stage :: Ours , 3 => Stage :: Theirs , _ => unreachable ! ("BUG: Flags::STAGE_MASK is two bits, whose 4 possible values we have covered") , } } # [doc = " Return an entry's stage as raw number between 0 and 4."] # [doc = " Possible values are:"] # [doc = ""] # [doc = " * 0 = no conflict,"] # [doc = " * 1 = base,"] # [doc = " * 2 = ours,"] # [doc = " * 3 = theirs"] pub fn stage_raw (& self) -> u32 { (* self & Flags :: STAGE_MASK) . bits () >> 12 } # [doc = " Transform ourselves to a storage representation to keep all flags which are to be persisted,"] # [doc = " skipping all extended flags. Note that the caller has to check for the `EXTENDED` bit to be present"] # [doc = " and write extended flags as well if so."] pub fn to_storage (mut self) -> at_rest :: Flags { at_rest :: Flags :: from_bits_retain ({ self . remove (Self :: PATH_LEN) ; self } . bits () as u16 ,) } }
    };
}

impl_88!()