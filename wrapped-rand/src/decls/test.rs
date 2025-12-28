macro_rules! deps {
    () => {
        RngReader!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { use super :: * ; # [doc = " Construct a deterministic RNG with the given seed"] pub fn rng (seed : u64) -> impl RngCore { const INC : u64 = 11634580027462260723 ; rand_pcg :: Pcg32 :: new (seed , INC) } # [doc = " Construct a generator yielding a constant value"] pub fn const_rng (x : u64) -> StepRng { StepRng (x , 0) } # [doc = " Construct a generator yielding an arithmetic sequence"] pub fn step_rng (x : u64 , increment : u64) -> StepRng { StepRng (x , increment) } # [derive (Clone)] pub struct StepRng (u64 , u64) ; impl RngCore for StepRng { fn next_u32 (& mut self) -> u32 { self . next_u64 () as u32 } fn next_u64 (& mut self) -> u64 { let res = self . 0 ; self . 0 = self . 0 . wrapping_add (self . 1) ; res } fn fill_bytes (& mut self , dst : & mut [u8]) { rand_core :: le :: fill_bytes_via_next (self , dst) } } # [cfg (feature = "std")] # [test] fn rng_reader () { use std :: io :: Read ; let mut rng = StepRng (255 , 1) ; let mut buf = [0u8 ; 24] ; let expected = [255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 ,] ; RngReader (& mut rng) . read_exact (& mut buf) . unwrap () ; assert_eq ! (& buf , & expected) ; RngReader (StepRng (255 , 1)) . read_exact (& mut buf) . unwrap () ; assert_eq ! (& buf , & expected) ; } # [test] # [cfg (feature = "thread_rng")] fn test_random () { let _n : u64 = random () ; let _f : f32 = random () ; # [allow (clippy :: type_complexity)] let _many : (() , [(u32 , bool) ; 3] , (u8 , i8 , u16 , i16 , u32 , i32 , u64 , i64) , (f32 , (f64 , (f64 ,))) ,) = random () ; } # [test] # [cfg (feature = "thread_rng")] fn test_range () { let _n : usize = random_range (42 ..= 43) ; let _f : f32 = random_range (42.0 .. 43.0) ; } }
    };
}

test!()