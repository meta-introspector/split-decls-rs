macro_rules! deps {
    () => {
        Hash!();
        State!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Hash { pub fn new () -> Hash { Hash { state : State :: new () , r : 0 , w : [0u8 ; 128] , len : 0 , } } # [doc = " Absorb content"] pub fn update < T : AsRef < [u8] > > (& mut self , input : T) { let input = input . as_ref () ; let mut n = input . len () ; self . len += n ; let av = 128 - self . r ; let tc = :: core :: cmp :: min (n , av) ; self . w [self . r .. self . r + tc] . copy_from_slice (& input [0 .. tc]) ; self . r += tc ; n -= tc ; let pos = tc ; if self . r == 128 { self . state . blocks (& self . w) ; self . r = 0 ; } if self . r == 0 && n > 0 { let rb = self . state . blocks (& input [pos ..]) ; if rb > 0 { self . w [.. rb] . copy_from_slice (& input [pos + n - rb ..]) ; self . r = rb ; } } } # [doc = " Compute SHA512(absorbed content)"] pub fn finalize (mut self) -> [u8 ; 64] { let mut padded = [0u8 ; 256] ; padded [.. self . r] . copy_from_slice (& self . w [.. self . r]) ; padded [self . r] = 0x80 ; let r = if self . r < 112 { 128 } else { 256 } ; let bits = self . len * 8 ; for i in 0 .. 8 { padded [r - 8 + i] = (bits as u64 >> (56 - i * 8)) as u8 ; } self . state . blocks (& padded [.. r]) ; let mut out = [0u8 ; 64] ; self . state . store (& mut out) ; out } # [doc = " Compute SHA512(`input`)"] pub fn hash < T : AsRef < [u8] > > (input : T) -> [u8 ; 64] { let mut h = Hash :: new () ; h . update (input) ; h . finalize () } }
    };
}

impl_54!()