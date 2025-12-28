macro_rules! deps {
    () => {
        Error!();
        Noise!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl Noise { # [doc = " Number of raw bytes for a noise component."] pub const BYTES : usize = 16 ; # [doc = " Creates a new noise component from raw bytes."] pub fn new (noise : [u8 ; Noise :: BYTES]) -> Self { Noise (noise) } # [doc = " Creates noise from a slice."] pub fn from_slice (noise : & [u8]) -> Result < Self , Error > { let mut noise_ = [0u8 ; Noise :: BYTES] ; if noise . len () != noise_ . len () { return Err (Error :: InvalidSeed) ; } noise_ . copy_from_slice (noise) ; Ok (Noise :: new (noise_)) } }
    };
}

impl_86!()