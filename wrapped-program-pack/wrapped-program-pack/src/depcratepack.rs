// Generated macro for Pack (trait)
macro_rules! DepcratePack {
() => {
// Module: crate
// Provides: {"Pack"}
// Dependencies: {}
# [doc = " Safely and efficiently (de)serialize account state"] pub trait Pack : Sealed { # [doc = " The length, in bytes, of the packed representation"] const LEN : usize ; # [doc (hidden)] fn pack_into_slice (& self , dst : & mut [u8]) ; # [doc (hidden)] fn unpack_from_slice (src : & [u8]) -> Result < Self , ProgramError > ; # [doc = " Get the packed length"] fn get_packed_len () -> usize { Self :: LEN } # [doc = " Unpack from slice and check if initialized"] fn unpack (input : & [u8]) -> Result < Self , ProgramError > where Self : IsInitialized , { let value = Self :: unpack_unchecked (input) ? ; if value . is_initialized () { Ok (value) } else { Err (ProgramError :: UninitializedAccount) } } # [doc = " Unpack from slice without checking if initialized"] fn unpack_unchecked (input : & [u8]) -> Result < Self , ProgramError > { if input . len () != Self :: LEN { return Err (ProgramError :: InvalidAccountData) ; } Self :: unpack_from_slice (input) } # [doc = " Pack into slice"] fn pack (src : Self , dst : & mut [u8]) -> Result < () , ProgramError > { if dst . len () != Self :: LEN { return Err (ProgramError :: InvalidAccountData) ; } src . pack_into_slice (dst) ; Ok (()) } }
};
}
