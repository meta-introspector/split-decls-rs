macro_rules! Hash234 {
    () => {
        pub struct Hash234 { hash2_table : Vec < i32 > , hash3_table : Vec < i32 > , hash4_table : Vec < i32 > , hash4_size : u32 , hash4_mask : u32 , hash2_value : i32 , hash3_value : i32 , hash4_value : i32 , }
    };
}

Hash234!()