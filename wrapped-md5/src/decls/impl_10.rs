macro_rules! deps {
    () => {
        Digest!();
        Context!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Context { # [doc = " Create a context for computing a digest."] # [inline] pub fn new () -> Context { Context { buffer : [0 ; 64] , count : 0 , state : [0x67452301 , 0xefcdab89 , 0x98badcfe , 0x10325476] , } } # [doc = " Consume data."] # [inline] pub fn consume < T : AsRef < [u8] > > (& mut self , data : T) { consume (self , data . as_ref ()) ; } # [doc = " Finalize and return the digest."] # [rustfmt :: skip] # [allow (clippy :: double_parens , clippy :: needless_range_loop)] pub fn finalize (mut self) -> Digest { let mut input = [0u32 ; 16] ; let k = ((self . count >> 3) & 0x3f) as usize ; input [14] = self . count as u32 ; input [15] = (self . count >> 32) as u32 ; consume (& mut self , & PADDING [.. (if k < 56 { 56 - k } else { 120 - k })] ,) ; let mut j = 0 ; for i in 0 .. 14 { input [i] = ((self . buffer [j + 3] as u32) << 24) | ((self . buffer [j + 2] as u32) << 16) | ((self . buffer [j + 1] as u32) << 8) | ((self . buffer [j] as u32)) ; j += 4 ; } transform (& mut self . state , & input) ; let mut digest = [0u8 ; 16] ; let mut j = 0 ; for i in 0 .. 4 { digest [j] = ((self . state [i]) & 0xff) as u8 ; digest [j + 1] = ((self . state [i] >> 8) & 0xff) as u8 ; digest [j + 2] = ((self . state [i] >> 16) & 0xff) as u8 ; digest [j + 3] = ((self . state [i] >> 24) & 0xff) as u8 ; j += 4 ; } Digest (digest) } # [doc = " Finalize and return the digest."] # [deprecated (since = "0.8.0" , note = "Use `finalize`.")] # [inline] pub fn compute (self) -> Digest { self . finalize () } }
    };
}

impl_10!()