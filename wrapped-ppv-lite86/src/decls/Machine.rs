macro_rules! deps {
    () => {
        Store!();
        StoreBytes!();
        MultiLane!();
    };
}

macro_rules! Machine {
    () => {
        deps!();
        pub trait Machine : Sized + Copy { type u32x4 : u32x4 < Self > ; type u64x2 : u64x2 < Self > ; type u128x1 : u128x1 < Self > ; type u32x4x2 : u32x4x2 < Self > ; type u64x2x2 : u64x2x2 < Self > ; type u64x4 : u64x4 < Self > ; type u128x2 : u128x2 < Self > ; type u32x4x4 : u32x4x4 < Self > ; type u64x2x4 : u64x2x4 < Self > ; type u128x4 : u128x4 < Self > ; # [inline (always)] fn unpack < S , V : Store < S > > (self , s : S) -> V { unsafe { V :: unpack (s) } } # [inline (always)] fn vec < V , A > (self , a : A) -> V where V : MultiLane < A > , { V :: from_lanes (a) } # [inline (always)] fn read_le < V > (self , input : & [u8]) -> V where V : StoreBytes , { unsafe { V :: unsafe_read_le (input) } } # [inline (always)] fn read_be < V > (self , input : & [u8]) -> V where V : StoreBytes , { unsafe { V :: unsafe_read_be (input) } } # [doc = " # Safety"] # [doc = " Caller must ensure the type of Self is appropriate for the hardware of the execution"] # [doc = " environment."] unsafe fn instance () -> Self ; }
    };
}

Machine!();