macro_rules! constant_time_eq_16 {
    () => {
        # [doc = " Compares two 128-bit byte strings in constant time."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use constant_time_eq::constant_time_eq_16;"] # [doc = ""] # [doc = " assert!(constant_time_eq_16(&[3; 16], &[3; 16]));"] # [doc = " assert!(!constant_time_eq_16(&[3; 16], &[7; 16]));"] # [doc = " ```"] # [inline] # [must_use] pub fn constant_time_eq_16 (a : & [u8 ; 16] , b : & [u8 ; 16]) -> bool { constant_time_eq_n (a , b) }
    };
}

constant_time_eq_16!()