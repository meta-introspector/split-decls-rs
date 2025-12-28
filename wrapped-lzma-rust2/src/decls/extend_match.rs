macro_rules! extend_match {
    () => {
        # [doc = " Extends a match to its maximum possible length within a specified limit."] # [inline (always)] fn extend_match (buf : & [u8] , read_pos : i32 , current_len : i32 , distance : i32 , limit : i32) -> i32 { let start1 = (read_pos + current_len) as usize ; let start2 = start1 - distance as usize ; # [cfg (not (feature = "optimization"))] let (s1 , s2) = { let extension_limit = (limit - current_len) as usize ; (& buf [start1 .. start1 + extension_limit] , & buf [start2 .. start2 + extension_limit] ,) } ; # [cfg (feature = "optimization")] let (s1 , s2) = unsafe { let logical_extension = (limit - current_len) as usize ; let physical_extension = buf . len () . saturating_sub (start1) ; let extension_limit = logical_extension . min (physical_extension) ; (buf . get_unchecked (start1 .. start1 + extension_limit) , buf . get_unchecked (start2 .. start2 + extension_limit) ,) } ; let extension = extend_match_safe (s1 , s2) as i32 ; current_len + extension }
    };
}

extend_match!()