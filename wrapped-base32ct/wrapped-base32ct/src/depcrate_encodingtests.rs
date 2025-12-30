// Generated macro for tests (module)
macro_rules! Depcrate_encodingtests {
() => {
// Module: crate::encoding
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "alloc"))] mod tests { use crate :: { Base32 , Base32Unpadded , Encoding } ; struct LenData { forty_bit_groups_len : usize , last_group_len : usize , padding_len : usize , } fn get_len_data (data_len : usize) -> LenData { let forty_bit_groups_len = data_len / 5 * 8 ; let (last_group_len , padding_len) = match data_len % 5 { 0 => (0 , 0) , 1 => (2 , 6) , 2 => (4 , 4) , 3 => (5 , 3) , 4 => (7 , 1) , _ => unreachable ! () , } ; LenData { forty_bit_groups_len , last_group_len , padding_len , } } # [test] fn unpadded_encoded_len () { let mut buf = vec ! [] ; assert_eq ! (Base32Unpadded :: encoded_len (& buf) , 0) ; for _ in 0 .. 10 { buf . push (b'a') ; let LenData { forty_bit_groups_len , last_group_len , padding_len : _ , } = get_len_data (buf . len ()) ; assert_eq ! (Base32Unpadded :: encoded_len (& buf) , forty_bit_groups_len + last_group_len) ; } } # [test] fn padded_encoded_len () { let mut buf = vec ! [] ; assert_eq ! (Base32 :: encoded_len (& buf) , 0) ; for _ in 0 .. 10 { buf . push (b'a') ; let LenData { forty_bit_groups_len , last_group_len , padding_len , } = get_len_data (buf . len ()) ; assert_eq ! (Base32 :: encoded_len (& buf) , forty_bit_groups_len + last_group_len + padding_len ,) ; } } }
};
}
